use oicnp_core::utils::{generate_snow_id, unique_id, uuid};

fn main() {
    let mut ids_36: Vec<String> = Vec::new();
    let mut ids_64: Vec<String> = Vec::new();
    let mut ids_uuid: Vec<String> = Vec::new();

    for _i in 0..100 {
        let (id, _raw_id) = unique_id(36);

        if let Some(target) = ids_36.iter().find(|item| item.as_str().eq(id.as_str())) {
            println!("------ {}", target);
        }
        ids_36.push(id)
    }

    for _i in 0..100 {
        let (id, _raw_id) = generate_snow_id(62);
        if let Some(target) = ids_64.iter().find(|item| item.as_str().eq(id.as_str())) {
            println!("===== {}", target);
        }
        ids_64.push(id)
    }

    for _i in 0..100 {
        let id = uuid();
        ids_uuid.push(id);
    }

    println!("{:?}", ids_36);
    println!("=======================");
    println!("{:?}", ids_64);
    println!("=======================");
    println!("{:?}", ids_uuid);
}