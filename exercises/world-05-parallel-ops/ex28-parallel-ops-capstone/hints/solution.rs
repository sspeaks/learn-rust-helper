use std::sync::{mpsc, Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineJob {
    pub job_id: String,
    pub payload: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineStageOutput {
    pub job_id: String,
    pub stage_one: i32,
    pub stage_two: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelPipelineReport {
    pub outputs: Vec<PipelineStageOutput>,
    pub checksum: i32,
}

#[derive(Debug)]
pub enum ParallelOpsError {
    WorkerPanicked,
    ChannelClosed,
    LockPoisoned,
}

pub fn run_parallel_pipeline(
    jobs: Vec<PipelineJob>,
) -> Result<ParallelPipelineReport, ParallelOpsError> {
    let job_count = jobs.len();
    if job_count == 0 {
        return Ok(ParallelPipelineReport {
            outputs: Vec::new(),
            checksum: 0,
        });
    }

    let checksum = Arc::new(Mutex::new(0_i32));
    let (tx, rx) = mpsc::channel::<(usize, PipelineStageOutput)>();
    let mut handles = Vec::with_capacity(job_count);

    for (index, job) in jobs.into_iter().enumerate() {
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || {
            let output = PipelineStageOutput {
                job_id: job.job_id,
                stage_one: job.payload * 2,
                stage_two: job.payload * 3,
            };

            tx.send((index, output))
                .map_err(|_| ParallelOpsError::ChannelClosed)
        }));
    }
    drop(tx);

    for handle in handles {
        let worker_outcome = handle
            .join()
            .map_err(|_| ParallelOpsError::WorkerPanicked)?;
        worker_outcome?;
    }

    let mut indexed_outputs = Vec::with_capacity(job_count);
    for _ in 0..job_count {
        let (index, output) = rx.recv().map_err(|_| ParallelOpsError::ChannelClosed)?;
        {
            let mut guard = checksum
                .lock()
                .map_err(|_| ParallelOpsError::LockPoisoned)?;
            *guard += output.stage_two;
        }
        indexed_outputs.push((index, output));
    }

    indexed_outputs.sort_by_key(|(index, _)| *index);
    let outputs = indexed_outputs
        .into_iter()
        .map(|(_, output)| output)
        .collect::<Vec<_>>();

    let checksum = *checksum
        .lock()
        .map_err(|_| ParallelOpsError::LockPoisoned)?;

    Ok(ParallelPipelineReport { outputs, checksum })
}
