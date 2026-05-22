use crate::config::Config;
use crate::constants::{INVALID_IDX, MAX_PASSWORD_BYTES, UNSET_INDEX};
use crate::error::Result;
use crate::model::{EncodedChar, Model};
use crate::output::Output;
use crate::route::Route;

pub(crate) fn generate(
    config: &Config,
    basechars: &[char],
    routes: &[Route],
    model: &Model,
    out: &mut Output,
) -> Result<()> {
    if !config.compat_order {
        return generate_fast(config, basechars, routes, model, out);
    }

    let base_count = basechars.len() as u64;
    if base_count == 0 {
        return Ok(());
    }

    let dist_count = config.dist_count();
    let mod_count = config.mod_count();
    let dir_count = config.dir_count();
    let selection_count = dist_count * mod_count * dir_count;
    let selection_count_u64 = selection_count as u64;

    let mut candidate = [0u8; MAX_PASSWORD_BYTES];

    for route in routes {
        let mut keyspace = base_count;
        for _ in 0..route.changes {
            keyspace = keyspace.wrapping_mul(selection_count_u64);
        }

        for k in 0..keyspace {
            let km = (k % base_count) as usize;
            let kd = k / base_count;
            let root = basechars[km];
            let root_idx = model.index_by_code[root as usize];

            if root_idx == UNSET_INDEX {
                continue;
            }

            let mut candidate_len = 0usize;
            append_encoded_unchecked(&mut candidate, &mut candidate_len, model.encoded[root_idx]);

            let mut current_idx = root_idx;
            let mut left = kd;
            let mut prev_selection = INVALID_IDX;
            let mut valid = true;

            'steps: for route_pos in 0..route.changes {
                if selection_count_u64 == 0 {
                    valid = false;
                    break;
                }

                let selection = (left % selection_count_u64) as usize;

                // Original variable is named prev_direction, but it compares the entire compact
                // distance+modifier+direction selection. Preserve that exact behavior.
                if prev_selection == selection {
                    valid = false;
                    break;
                }
                prev_selection = selection;

                for _ in 0..route.repeat[route_pos] {
                    let next_idx = model.states[current_idx].next[selection];
                    if next_idx == INVALID_IDX {
                        valid = false;
                        break 'steps;
                    }

                    append_encoded_unchecked(
                        &mut candidate,
                        &mut candidate_len,
                        model.encoded[next_idx],
                    );
                    current_idx = next_idx;
                }

                left /= selection_count_u64;
            }

            if valid {
                out.push_line(&candidate[..candidate_len])?;
            }
        }
    }

    out.flush()?;
    Ok(())
}

#[inline(always)]
pub(crate) fn generate_fast(
    config: &Config,
    basechars: &[char],
    routes: &[Route],
    model: &Model,
    out: &mut Output,
) -> Result<()> {
    if basechars.is_empty() {
        return Ok(());
    }

    let selection_count = config.dist_count() * config.mod_count() * config.dir_count();
    if selection_count == 0 {
        return Ok(());
    }

    let mut candidate = [0u8; MAX_PASSWORD_BYTES];

    for route in routes {
        let mut ctx = FastRouteCtx {
            route,
            model,
            selection_count,
            candidate: &mut candidate,
            out,
        };

        for &root in basechars {
            let root_idx = model.index_by_code[root as usize];
            if root_idx == UNSET_INDEX {
                continue;
            }

            let mut candidate_len = 0usize;
            append_encoded_unchecked(ctx.candidate, &mut candidate_len, model.encoded[root_idx]);

            ctx.generate(0, INVALID_IDX, root_idx, candidate_len)?;
        }
    }

    out.flush()?;
    Ok(())
}

struct FastRouteCtx<'a> {
    route: &'a Route,
    model: &'a Model,
    selection_count: usize,
    candidate: &'a mut [u8; MAX_PASSWORD_BYTES],
    out: &'a mut Output,
}

impl FastRouteCtx<'_> {
    #[inline(always)]
    pub(crate) fn generate(
        &mut self,
        route_pos: usize,
        prev_selection: usize,
        current_idx: usize,
        candidate_len: usize,
    ) -> Result<()> {
        if route_pos == self.route.changes {
            self.out.push_line(&self.candidate[..candidate_len])?;
            return Ok(());
        }

        for selection in 0..self.selection_count {
            if selection == prev_selection {
                continue;
            }

            let mut next_current_idx = current_idx;
            let mut next_candidate_len = candidate_len;
            let mut valid = true;

            for _ in 0..self.route.repeat[route_pos] {
                let next_idx = self.model.states[next_current_idx].next[selection];
                if next_idx == INVALID_IDX {
                    valid = false;
                    break;
                }

                append_encoded_unchecked(
                    self.candidate,
                    &mut next_candidate_len,
                    self.model.encoded[next_idx],
                );
                next_current_idx = next_idx;
            }

            if !valid {
                continue;
            }

            self.generate(
                route_pos + 1,
                selection,
                next_current_idx,
                next_candidate_len,
            )?;
        }

        Ok(())
    }
}

#[inline(always)]
fn append_encoded_unchecked(
    buf: &mut [u8; MAX_PASSWORD_BYTES],
    pos: &mut usize,
    encoded: EncodedChar,
) {
    let len = encoded.len as usize;

    // SAFETY: parser caps route length at 32 and repeat at 16, therefore a candidate is at most
    // 513 Unicode scalar values. MAX_PASSWORD_BYTES is 513 * 4 bytes. EncodedChar is always <= 4 bytes.
    unsafe {
        std::ptr::copy_nonoverlapping(encoded.bytes.as_ptr(), buf.as_mut_ptr().add(*pos), len);
    }

    *pos += len;
}
