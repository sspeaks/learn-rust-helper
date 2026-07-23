use std::sync::mpsc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoJob {
    pub cargo_id: String,
    pub destination: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoReceipt {
    pub cargo_id: String,
    pub delivered_to: String,
}

#[derive(Debug)]
pub enum CargoChannelError {
    SendFailed,
    ReceiveFailed,
    WorkerPanicked,
}

pub fn dispatch_cargo_jobs(jobs: Vec<CargoJob>) -> Result<Vec<CargoReceipt>, CargoChannelError> {
    let job_count = jobs.len();
    let (tx, rx) = mpsc::channel::<(usize, CargoReceipt)>();
    let mut handles = Vec::with_capacity(job_count);

    for (index, job) in jobs.into_iter().enumerate() {
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || {
            let receipt = CargoReceipt {
                cargo_id: job.cargo_id,
                delivered_to: job.destination,
            };

            tx.send((index, receipt))
                .map_err(|_| CargoChannelError::SendFailed)
        }));
    }
    drop(tx);

    for handle in handles {
        let worker_outcome = handle
            .join()
            .map_err(|_| CargoChannelError::WorkerPanicked)?;
        worker_outcome?;
    }

    let mut indexed = Vec::with_capacity(job_count);
    for _ in 0..job_count {
        indexed.push(rx.recv().map_err(|_| CargoChannelError::ReceiveFailed)?);
    }

    indexed.sort_by_key(|(index, _)| *index);
    Ok(indexed.into_iter().map(|(_, receipt)| receipt).collect())
}
