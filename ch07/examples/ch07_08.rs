use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, RwLock};

fn main() -> Result<(), Box<dyn Error>> {
    let dogs: HashSet<_> = ["紫", "トイプードル"].iter().cloned().collect();
    let dogs = Arc::new(RwLock::new(dogs));

    fn stringify(x: impl ToString) -> String { x.to_string() }
    {
        let ds = dogs.read().map_err(stringify)?;
        assert!(ds.contains("紫"));
        assert!(ds.contains("トイプードル"));
    }

    dogs.write().map_err(stringify)?.insert("ブルーテリア");

    let dogs1 = Arc::clone(&dogs);
    std::thread::spawn(move ||
        dogs1.write().map(|mut ds| ds.insert("コーギー")).map_err(stringify)
    ).join().expect("Thread error")?;

    assert!(dogs.read().map_err(stringify)?.contains("ブルーテリア"));
    assert!(dogs.read().map_err(stringify)?.contains("コーギー"));
    Ok(())
}