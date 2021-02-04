use super::*;
use futures_channel::oneshot;
use futures_channel::oneshot::Sender;
use std::sync::mpsc;
use std::thread;

/// An action the database thread can perform.
pub enum Action {
    UpdatePerson(Person, Sender<DatabaseResult<()>>),
    GetPerson(String, Sender<DatabaseResult<Option<Person>>>),
    DeletePerson(String, Sender<DatabaseResult<()>>),
    GetPersons(Sender<DatabaseResult<Vec<Person>>>),
    UpdateInstrument(Instrument, Sender<DatabaseResult<()>>),
    GetInstrument(String, Sender<DatabaseResult<Option<Instrument>>>),
    DeleteInstrument(String, Sender<DatabaseResult<()>>),
    GetInstruments(Sender<DatabaseResult<Vec<Instrument>>>),
    UpdateWork(Work, Sender<DatabaseResult<()>>),
    DeleteWork(String, Sender<DatabaseResult<()>>),
    GetWorks(String, Sender<DatabaseResult<Vec<Work>>>),
    UpdateEnsemble(Ensemble, Sender<DatabaseResult<()>>),
    GetEnsemble(String, Sender<DatabaseResult<Option<Ensemble>>>),
    DeleteEnsemble(String, Sender<DatabaseResult<()>>),
    GetEnsembles(Sender<DatabaseResult<Vec<Ensemble>>>),
    UpdateRecording(Recording, Sender<DatabaseResult<()>>),
    DeleteRecording(String, Sender<DatabaseResult<()>>),
    GetRecordingsForPerson(String, Sender<DatabaseResult<Vec<Recording>>>),
    GetRecordingsForEnsemble(String, Sender<DatabaseResult<Vec<Recording>>>),
    GetRecordingsForWork(String, Sender<DatabaseResult<Vec<Recording>>>),
    RecordingExists(String, Sender<DatabaseResult<bool>>),
    UpdateMedium(Medium, Sender<DatabaseResult<()>>),
    GetMedium(String, Sender<DatabaseResult<Option<Medium>>>),
    DeleteMedium(String, Sender<DatabaseResult<()>>),
    GetTrackSets(String, Sender<DatabaseResult<Vec<TrackSet>>>),
    Stop(Sender<()>),
}

use Action::*;

/// A database running within a thread.
pub struct DbThread {
    action_sender: mpsc::Sender<Action>,
}

impl DbThread {
    /// Create a new database connection in a background thread.
    pub async fn new(path: String) -> DatabaseResult<Self> {
        let (action_sender, action_receiver) = mpsc::channel();
        let (ready_sender, ready_receiver) = oneshot::channel();

        thread::spawn(move || {
            let db = match Database::new(&path) {
                Ok(db) => {
                    ready_sender.send(Ok(())).unwrap();
                    db
                }
                Err(error) => {
                    ready_sender.send(Err(error)).unwrap();
                    return;
                }
            };

            for action in action_receiver {
                match action {
                    UpdatePerson(person, sender) => {
                        sender.send(db.update_person(person)).unwrap();
                    }
                    GetPerson(id, sender) => {
                        sender.send(db.get_person(&id)).unwrap();
                    }
                    DeletePerson(id, sender) => {
                        sender.send(db.delete_person(&id)).unwrap();
                    }
                    GetPersons(sender) => {
                        sender.send(db.get_persons()).unwrap();
                    }
                    UpdateInstrument(instrument, sender) => {
                        sender.send(db.update_instrument(instrument)).unwrap();
                    }
                    GetInstrument(id, sender) => {
                        sender.send(db.get_instrument(&id)).unwrap();
                    }
                    DeleteInstrument(id, sender) => {
                        sender.send(db.delete_instrument(&id)).unwrap();
                    }
                    GetInstruments(sender) => {
                        sender.send(db.get_instruments()).unwrap();
                    }
                    UpdateWork(work, sender) => {
                        sender.send(db.update_work(work)).unwrap();
                    }
                    DeleteWork(id, sender) => {
                        sender.send(db.delete_work(&id)).unwrap();
                    }
                    GetWorks(id, sender) => {
                        sender.send(db.get_works(&id)).unwrap();
                    }
                    UpdateEnsemble(ensemble, sender) => {
                        sender.send(db.update_ensemble(ensemble)).unwrap();
                    }
                    GetEnsemble(id, sender) => {
                        sender.send(db.get_ensemble(&id)).unwrap();
                    }
                    DeleteEnsemble(id, sender) => {
                        sender.send(db.delete_ensemble(&id)).unwrap();
                    }
                    GetEnsembles(sender) => {
                        sender.send(db.get_ensembles()).unwrap();
                    }
                    UpdateRecording(recording, sender) => {
                        sender.send(db.update_recording(recording)).unwrap();
                    }
                    DeleteRecording(id, sender) => {
                        sender.send(db.delete_recording(&id)).unwrap();
                    }
                    GetRecordingsForPerson(id, sender) => {
                        sender.send(db.get_recordings_for_person(&id)).unwrap();
                    }
                    GetRecordingsForEnsemble(id, sender) => {
                        sender.send(db.get_recordings_for_ensemble(&id)).unwrap();
                    }
                    GetRecordingsForWork(id, sender) => {
                        sender.send(db.get_recordings_for_work(&id)).unwrap();
                    }
                    RecordingExists(id, sender) => {
                        sender.send(db.recording_exists(&id)).unwrap();
                    }
                    UpdateMedium(medium, sender) => {
                        sender.send(db.update_medium(medium)).unwrap();
                    }
                    GetMedium(id, sender) => {
                        sender.send(db.get_medium(&id)).unwrap();
                    }
                    DeleteMedium(id, sender) => {
                        sender.send(db.delete_medium(&id)).unwrap();
                    }
                    GetTrackSets(recording_id, sender) => {
                        sender.send(db.get_track_sets(&recording_id)).unwrap();
                    }
                    Stop(sender) => {
                        sender.send(()).unwrap();
                        break;
                    }
                }
            }
        });

        ready_receiver.await??;
        Ok(Self { action_sender })
    }

    /// Update an existing person or insert a new one.
    pub async fn update_person(&self, person: Person) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender.send(UpdatePerson(person, sender))?;
        receiver.await?
    }

    /// Get an existing person.
    pub async fn get_person(&self, id: &str) -> DatabaseResult<Option<Person>> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender.send(GetPerson(id.to_string(), sender))?;
        receiver.await?
    }

    /// Delete an existing person. This will fail, if there are still other items referencing
    /// this person.
    pub async fn delete_person(&self, id: &str) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(DeletePerson(id.to_string(), sender))?;
        receiver.await?
    }

    /// Get all existing persons.
    pub async fn get_persons(&self) -> DatabaseResult<Vec<Person>> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender.send(GetPersons(sender))?;
        receiver.await?
    }

    /// Update an existing instrument or insert a new one.
    pub async fn update_instrument(&self, instrument: Instrument) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(UpdateInstrument(instrument, sender))?;
        receiver.await?
    }

    /// Get an existing instrument.
    pub async fn get_instrument(&self, id: &str) -> DatabaseResult<Option<Instrument>> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(GetInstrument(id.to_string(), sender))?;
        receiver.await?
    }

    /// Delete an existing instrument. This will fail, if there are still other items referencing
    /// this instrument.
    pub async fn delete_instrument(&self, id: &str) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(DeleteInstrument(id.to_string(), sender))?;
        receiver.await?
    }

    /// Get all existing instruments.
    pub async fn get_instruments(&self) -> DatabaseResult<Vec<Instrument>> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender.send(GetInstruments(sender))?;
        receiver.await?
    }

    /// Update an existing work or insert a new one.
    pub async fn update_work(&self, work: Work) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender.send(UpdateWork(work, sender))?;
        receiver.await?
    }

    /// Delete an existing work. This will fail, if there are still other items referencing
    /// this work.
    pub async fn delete_work(&self, id: &str) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(DeleteWork(id.to_string(), sender))?;
        receiver.await?
    }

    /// Get information on all existing works by a composer.
    pub async fn get_works(&self, person_id: &str) -> DatabaseResult<Vec<Work>> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(GetWorks(person_id.to_string(), sender))?;
        receiver.await?
    }

    /// Update an existing ensemble or insert a new one.
    pub async fn update_ensemble(&self, ensemble: Ensemble) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender.send(UpdateEnsemble(ensemble, sender))?;
        receiver.await?
    }

    /// Get an existing ensemble.
    pub async fn get_ensemble(&self, id: &str) -> DatabaseResult<Option<Ensemble>> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(GetEnsemble(id.to_string(), sender))?;
        receiver.await?
    }

    /// Delete an existing ensemble. This will fail, if there are still other items referencing
    /// this ensemble.
    pub async fn delete_ensemble(&self, id: &str) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(DeleteEnsemble(id.to_string(), sender))?;
        receiver.await?
    }

    /// Get all existing ensembles.
    pub async fn get_ensembles(&self) -> DatabaseResult<Vec<Ensemble>> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender.send(GetEnsembles(sender))?;
        receiver.await?
    }

    /// Update an existing recording or insert a new one.
    pub async fn update_recording(&self, recording: Recording) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(UpdateRecording(recording, sender))?;
        receiver.await?
    }

    /// Delete an existing recording.
    pub async fn delete_recording(&self, id: &str) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(DeleteRecording(id.to_string(), sender))?;
        receiver.await?
    }

    /// Get information on all recordings in which a person performs.
    pub async fn get_recordings_for_person(&self, person_id: &str) -> DatabaseResult<Vec<Recording>> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(GetRecordingsForPerson(person_id.to_string(), sender))?;
        receiver.await?
    }

    /// Get information on all recordings in which an ensemble performs.
    pub async fn get_recordings_for_ensemble(&self, ensemble_id: &str) -> DatabaseResult<Vec<Recording>> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(GetRecordingsForEnsemble(ensemble_id.to_string(), sender))?;
        receiver.await?
    }

    /// Get information on all recordings of a work.
    pub async fn get_recordings_for_work(&self, work_id: &str) -> DatabaseResult<Vec<Recording>> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(GetRecordingsForWork(work_id.to_string(), sender))?;
        receiver.await?
    }

    /// Check whether a recording exists within the database.
    pub async fn recording_exists(&self, id: &str) -> DatabaseResult<bool> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender
            .send(RecordingExists(id.to_string(), sender))?;
        receiver.await?
    }

    /// Update an existing medium or insert a new one.
    pub async fn update_medium(&self, medium: Medium) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender.send(UpdateMedium(medium, sender))?;
        receiver.await?
    }

    /// Delete an existing medium. This will fail, if there are still other
    /// items referencing this medium.
    pub async fn delete_medium(&self, id: &str) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();

        self.action_sender
            .send(DeleteMedium(id.to_owned(), sender))?;

        receiver.await?
    }

    /// Get an existing medium.
    pub async fn get_medium(&self, id: &str) -> DatabaseResult<Option<Medium>> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender.send(GetMedium(id.to_owned(), sender))?;
        receiver.await?
    }

    /// Get all track sets for a recording.
    pub async fn get_track_sets(&self, recording_id: &str) -> DatabaseResult<Vec<TrackSet>> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender.send(GetTrackSets(recording_id.to_owned(), sender))?;
        receiver.await?
    }

    /// Stop the database thread. Any future access to the database will fail.
    pub async fn stop(&self) -> DatabaseResult<()> {
        let (sender, receiver) = oneshot::channel();
        self.action_sender.send(Stop(sender))?;
        Ok(receiver.await?)
    }
}
