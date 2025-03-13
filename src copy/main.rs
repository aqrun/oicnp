
mod controllers;

use zino::prelude::*;

pub fn main() {
    zino::Cluster::boot()
        .register(controllers::routes())
        // .register_debug(router::debug_routes())
        // .spawn(schedule::job_scheduler())
        // .run_with(schedule::async_job_scheduler())
        .run();
}
