#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrewTask {
    pub crew_id: String,
    pub units: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrewTaskResult {
    pub crew_id: String,
    pub delivered_units: u32,
}

#[derive(Debug)]
pub enum CrewDispatchError {
    WorkerPanicked,
}

pub fn run_crew_dispatch(tasks: Vec<CrewTask>) -> Result<Vec<CrewTaskResult>, CrewDispatchError> {
    let mut handles = Vec::with_capacity(tasks.len());

    for task in tasks {
        handles.push(std::thread::spawn(move || CrewTaskResult {
            crew_id: task.crew_id,
            delivered_units: task.units,
        }));
    }

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        let result = handle
            .join()
            .map_err(|_| CrewDispatchError::WorkerPanicked)?;
        results.push(result);
    }

    Ok(results)
}
