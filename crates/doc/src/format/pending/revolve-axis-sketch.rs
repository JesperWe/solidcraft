//! Solid and surface revolves can save an axis as `sketch_curve` with its owning sketch id
//! and line id. Older designs keep their existing profile-local `sketch_line` interpretation;
//! no migration is needed. The last released app refuses this newer format instead of losing
//! the source sketch reference.
