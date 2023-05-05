var play_history;
var play_queue;
var audio;
var audio_info;
var playlist_list;

function play_songs_next_in_play_queue(songs) {
    for (const song of songs.reverse()) {
        play_queue.unshift(song)
    }
}

function append_songs_to_play_queue(songs) {
    for (const song of songs) {
        play_queue.push(song)
    }
}

function play_next_song_in_queue() {
    const finished_src = audio.getAttribute('src');
    const finished_type = audio.getAttribute('type');
    const finished_title = audio_info.textContent;
    if (finished_src !== null) {
        play_history.push({src: finished_src, title: finished_title, contenttype: finished_type });
    }
    const next = play_queue.shift();
    if (next !== undefined) {
        audio.setAttribute('src', next.src);
        audio.setAttribute('type', next.contenttype);
        audio.setAttribute('preload', 'auto');
        audio_info.textContent = next.title;
        audio.load();
        audio.play();
    }
};

function play_prev_song_in_history() {
    const old_src = audio.getAttribute('src');
    const old_type = audio.getAttribute('type');
    const old_title = audio_info.textContent;
    if (old_src !== null) {
        play_queue.unshift({src: old_src, title: old_title, contenttype: old_type });
    }
    const next = play_history.pop();
    if (next !== undefined) {
        audio.setAttribute('src', next.src);
        audio.setAttribute('type', next.contenttype);
        audio.setAttribute('preload', 'auto');
        audio_info.textContent = next.title;
        audio.load();
        audio.play();
    }
};

window.onload = () => {
    play_history = [];
    play_queue = [];
    audio = document.getElementById('main-playback');
    audio_info = document.getElementById('main-playback-info');
    audio.addEventListener('ended', (event) => {
        console.log("just got this event", event)
        play_next_song_in_queue();
    }, false);

    playlist_list = document.getElementById('playlist-list');

    console.log('loaded');
};
