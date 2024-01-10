#![allow(async_fn_in_trait)]
#![allow(stable_features)]
#![feature(async_fn_in_trait)]
#![feature(lazy_cell)]
#![feature(let_chains)]

mod controllers;
mod models;
mod routers;
mod schedules;

use zino::prelude::*;

fn main() {
    zino::Cluster::boot()
        .register(routers::routes())
        // .register_debug(router::debug_routes())
        // .spawn(schedule::job_scheduler())
        .run_with(schedules::async_job_scheduler());
}
