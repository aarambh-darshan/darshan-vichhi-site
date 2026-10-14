// Loads the intro video only when the visitor asks for it. Nothing here runs at page load except
// showing the play button. The WebAssembly module (~0.3 MB) and the narration (2.7 MB) start
// downloading when the visitor hovers, focuses or touches the button.
const button = document.getElementById('intro-play');
const supported = button && 'WebAssembly' in window && !!document.createElement('canvas').getContext;

if (supported) {
  button.hidden = false;
  const base = new URL('assets/video/', document.baseURI);
  // A tiny silent clip: playing it inside the click "unlocks" the audio element on Safari / iOS.
  const SILENT = 'data:audio/wav;base64,UklGRigAAABXQVZFZm10IBIAAAABAAEARKwAAIhYAQACABAAAABkYXRhAgAAAAEA';

  let moduleLoading = null;
  let audioLoading = null;

  const loadModule = () =>
    (moduleLoading ??= import(new URL('intro_video.js', base).href).then(async (mod) => {
      await mod.default({ module_or_path: new URL('intro_video_bg.wasm', base) });
      return mod;
    }));

  // The narration is fetched whole and played from memory (a blob URL). That makes seeking work on
  // every server: a plain static server without range-request support cannot seek a streamed file.
  const loadAudio = () =>
    (audioLoading ??= fetch(new URL('intro.mp3', base))
      .then((response) => {
        if (!response.ok) throw new Error(`narration: HTTP ${response.status}`);
        return response.blob();
      })
      .then((blob) => URL.createObjectURL(blob)));

  const warm = () => {
    loadModule().catch(() => {});
    loadAudio().catch(() => {});
  };
  button.addEventListener('pointerenter', warm, { once: true });
  button.addEventListener('focus', warm, { once: true });
  button.addEventListener('touchstart', warm, { once: true, passive: true });

  button.addEventListener('click', async () => {
    if (button.classList.contains('is-loading')) return;
    button.classList.remove('is-returning');
    button.classList.add('is-loading');

    // Create the audio element and play (silence) inside the click gesture, so the browser allows
    // the narration to start later.
    const audio = new Audio();
    audio.src = SILENT;
    const unlocked = audio.play().catch(() => {});

    try {
      const [mod, url] = await Promise.all([loadModule(), loadAudio().catch(() => new URL('intro.mp3', base).href)]);
      await unlocked;
      audio.pause();
      audio.src = url;
      audio.preload = 'auto';
      audio.load();
      mod.open_intro(button, audio);
    } catch (error) {
      console.error('Could not start the intro video', error);
      audio.pause();
    } finally {
      button.classList.remove('is-loading');
    }
  });
}
