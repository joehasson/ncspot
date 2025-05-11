use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use crate::library::Library;
use crate::model::{
    album::Album, artist::Artist, episode::Episode, playable::Playable, track::Track,
};
use crate::queue::Queue;
use crate::traits::{ListItem, ViewExt};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifiedPlayable(Playable);

impl fmt::Display for VerifiedPlayable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.deref(), f)
    }
}

#[derive(Debug)]
pub struct UnplayableError;

impl TryFrom<Playable> for VerifiedPlayable {
    type Error = UnplayableError;

    fn try_from(playable: Playable) -> Result<Self, Self::Error> {
        if playable.is_playable() {
            Ok(VerifiedPlayable(playable))
        } else {
            Err(UnplayableError)
        }
    }
}

impl TryFrom<Track> for VerifiedPlayable {
    type Error = UnplayableError;

    fn try_from(track: Track) -> Result<Self, Self::Error> {
        if track.is_playable() {
            Ok(Self(Playable::Track(track)))
        } else {
            Err(UnplayableError)
        }
    }
}

impl From<Episode> for VerifiedPlayable {
    fn from(episode: Episode) -> Self {
        Self(Playable::Episode(episode))
    }
}

impl Deref for VerifiedPlayable {
    type Target = Playable;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ListItem for VerifiedPlayable {
    fn is_playing(&self, queue: &Queue) -> bool {
        self.0.is_playing(queue)
    }
    fn display_left(&self, library: &Library) -> String {
        self.0.display_left(library)
    }
    fn display_center(&self, library: &Library) -> String {
        self.0.display_center(library)
    }
    fn display_right(&self, library: &Library) -> String {
        self.0.display_right(library)
    }
    fn play(&mut self, queue: &Queue) {
        self.0.play(queue)
    }
    fn play_next(&mut self, queue: &Queue) {
        self.0.play_next(queue)
    }
    fn queue(&mut self, queue: &Queue) {
        self.0.queue(queue)
    }
    fn toggle_saved(&mut self, library: &Library) {
        self.0.toggle_saved(library)
    }
    fn save(&mut self, library: &Library) {
        self.0.save(library)
    }
    fn unsave(&mut self, library: &Library) {
        self.0.unsave(library)
    }
    fn open(&self, queue: Arc<Queue>, library: Arc<Library>) -> Option<Box<dyn ViewExt>> {
        self.0.open(queue, library)
    }
    fn open_recommendations(
        &mut self,
        queue: Arc<Queue>,
        library: Arc<Library>,
    ) -> Option<Box<dyn ViewExt>> {
        self.0.open_recommendations(queue, library)
    }
    fn share_url(&self) -> Option<String> {
        self.0.share_url()
    }

    fn album(&self, queue: &Queue) -> Option<Album> {
        self.0.album(queue)
    }

    fn artists(&self) -> Option<Vec<Artist>> {
        self.0.artists()
    }

    fn track(&self) -> Option<Track> {
        self.0.track()
    }

    fn is_saved(&self, library: &Library) -> Option<bool> {
        self.0.is_saved(library)
    }

    #[inline]
    fn is_playable(&self) -> bool {
        true
    }

    fn as_listitem(&self) -> Box<dyn ListItem> {
        self.0.as_listitem()
    }
}
