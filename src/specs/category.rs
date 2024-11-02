/*
 * MIT License
 *
 * Copyright (c) 2023-2024 Didier Plaindoux
 */

use crate::specs::identity::Identity;
use crate::specs::semigroupoid::Semigroupoid;

pub trait Category: Semigroupoid + Identity {
    /*
     * Laws:
     * - compose(id,f) == f
     * - compose(f,id) == f
     */
}
