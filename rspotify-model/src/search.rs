//! All object related to search

use serde::{Deserialize, Serialize};

use crate::{
    FullArtist, FullTrack, Page, SimplifiedAlbum, SimplifiedEpisode, SimplifiedPlaylist,
    SimplifiedShow,
};

/// Search for playlists
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SearchPlaylists {
    pub playlists: Page<SimplifiedPlaylist>,
}

/// Search for albums
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchAlbums {
    pub albums: Page<SimplifiedAlbum>,
}

/// Search for artists
///
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SearchArtists {
    pub artists: Page<FullArtist>,
}

/// Search item
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SearchTracks {
    pub tracks: Page<FullTrack>,
}

/// Search for shows
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SearchShows {
    pub shows: Page<SimplifiedShow>,
}

/// Search for episodes
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SearchEpisodes {
    pub episodes: Page<SimplifiedEpisode>,
}

/// Search result of any kind
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SearchResult {
    #[serde(rename = "playlists")]
    Playlists(Page<SimplifiedPlaylist>),
    #[serde(rename = "albums")]
    Albums(Page<SimplifiedAlbum>),
    #[serde(rename = "artists")]
    Artists(Page<FullArtist>),
    #[serde(rename = "tracks")]
    Tracks(Page<FullTrack>),
    #[serde(rename = "shows")]
    Shows(Page<SimplifiedShow>),
    #[serde(rename = "episodes")]
    Episodes(Page<SimplifiedEpisode>),
}
