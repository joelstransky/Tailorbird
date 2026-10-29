// Modern Indie-Pop / SaaS Launch Video Soundtrack (124 BPM)
// Fresh, crisp, upbeat, and bouncy:
// - Warm lush electric piano / Rhodes chords
// - Bouncy walking bassline
// - Modern punchy drum groove (kick, snappy clap/rimshot, 16th shakers, snaps)
// - Cheerful wooden marimba & sparkle bell arpeggios
// - Birdsong whistle flourishes
// - Dynamic build-ups and scene transition hits

import {mkdirSync, writeFileSync} from 'node:fs';

const SR = 44100;
const DURATION = 30;
const BPM = 124;
const BEAT = 60 / BPM; // ~0.48387s per beat
mkdirSync('public/audio', {recursive: true});

let seed = 998877;
const rnd = () => ((seed = (seed * 1664525 + 1013904223) >>> 0) / 4294967296) * 2 - 1;

function wav(name, samples) {
  const buf = Buffer.alloc(44 + samples.length * 2);
  buf.write('RIFF', 0);
  buf.writeUInt32LE(36 + samples.length * 2, 4);
  buf.write('WAVEfmt ', 8);
  buf.writeUInt32LE(16, 16);
  buf.writeUInt16LE(1, 20); // PCM
  buf.writeUInt16LE(1, 22); // Mono
  buf.writeUInt32LE(SR, 24);
  buf.writeUInt32LE(SR * 2, 28);
  buf.writeUInt16LE(2, 32);
  buf.writeUInt16LE(16, 34);
  buf.write('data', 36);
  buf.writeUInt32LE(samples.length * 2, 40);
  for (let i = 0; i < samples.length; i++) {
    buf.writeInt16LE(Math.max(-1, Math.min(1, samples[i])) * 32767, 44 + i * 2);
  }
  writeFileSync(`public/audio/${name}.wav`, buf);
}

const N = SR * DURATION;
const drums = new Float32Array(N);
const melodic = new Float32Array(N);
const chords = new Float32Array(N);
const bassTrack = new Float32Array(N);
const sfx = new Float32Array(N);

function mix(bus, startSec, arr, gain = 1) {
  const s0 = Math.floor(startSec * SR);
  for (let i = 0; i < arr.length && s0 + i < N; i++) {
    bus[s0 + i] += arr[i] * gain;
  }
}

const gen = (dur, fn) => Float32Array.from({length: Math.floor(dur * SR)}, (_, i) => fn(i / SR, i));

// --- Synthesizers ---

// Modern punchy kick
const modernKick = () => gen(0.3, (t) => {
  const f = 48 + 120 * Math.exp(-t * 36);
  const click = Math.sin(2 * Math.PI * 1800 * t) * Math.exp(-t * 120) * 0.3;
  return (Math.sin(2 * Math.PI * f * t) * Math.exp(-t * 10) + click) * 0.95;
});

// Crisp modern clap & rimshot
const snappyClap = () => gen(0.2, (t) => {
  const burst = [0, 0.009, 0.018].reduce((a, o) => a + (t > o ? Math.exp(-(t - o) * 90) : 0), 0);
  const rim = Math.sin(2 * Math.PI * 950 * t) * Math.exp(-t * 50) * 0.45;
  return (rnd() * burst * 0.5 + rim + rnd() * Math.exp(-t * 26) * 0.2) * 0.85;
});

// Crisp stereo shaker
const modernShaker = (acc = 0.15) => {
  let prev = 0;
  return gen(0.05, (t) => {
    const n = rnd();
    const hp = n - prev;
    prev = n;
    return hp * Math.exp(-t * 65) * acc;
  });
};

// Clean UI Pop / Click
const uiPop = (f = 1200) => gen(0.06, (t) => {
  return Math.sin(2 * Math.PI * (f + 400 * Math.exp(-t * 70)) * t) * Math.exp(-t * 60) * 0.5;
});

// Whoosh transition sweep
const whooshSweep = (dur = 0.45, up = true) => {
  let lp = 0;
  return gen(dur, (t) => {
    const rel = up ? t / dur : 1 - t / dur;
    lp += (0.05 + 0.6 * rel) * (rnd() - lp);
    return lp * Math.sin(Math.PI * (t / dur)) ** 1.8 * 1.5;
  });
};

// Birdsong chirp flourish
const birdWarble = (f1, f2, dur = 0.12) => gen(dur, (t) => {
  const f = f1 + (f2 - f1) * (t / dur) + Math.sin(2 * Math.PI * 40 * t) * 70;
  const env = Math.sin(Math.PI * (t / dur)) ** 2;
  return Math.sin(2 * Math.PI * f * t) * env * 0.4;
});

// Lush Electric Piano / Rhodes tone
const rhodesTine = (f, dur) => gen(dur, (t) => {
  const fundamental = Math.sin(2 * Math.PI * f * t) * Math.exp(-t * 1.2);
  const overtone = Math.sin(2 * Math.PI * f * 2.01 * t) * Math.exp(-t * 3.5) * 0.35;
  const tine = Math.sin(2 * Math.PI * f * 6.05 * t) * Math.exp(-t * 24) * 0.2;
  return (fundamental + overtone + tine) * Math.min(1, t * 250) * 0.5;
});

// Bouncy marimba arpeggio
const bouncyMarimba = (f, dur = 0.6) => gen(dur, (t) => {
  const fundamental = Math.sin(2 * Math.PI * f * t) * Math.exp(-t * 7);
  const wood = Math.sin(2 * Math.PI * f * 4 * t) * Math.exp(-t * 30) * 0.35;
  return (fundamental + wood) * Math.min(1, t * 600) * 0.8;
});

// Bell sparkle
const chimeSparkle = (f) => gen(1.5, (t) => {
  return (
    Math.sin(2 * Math.PI * f * t) +
    Math.sin(2 * Math.PI * f * 2.76 * t) * 0.4 +
    Math.sin(2 * Math.PI * f * 5.4 * t) * 0.15
  ) * Math.exp(-t * 3.0) * 0.45;
});

// Modern Bouncy Synth Bass
const modernBass = (f, dur) => gen(dur, (t) => {
  const saw = ((f * t) % 1) * 2 - 1;
  const sine = Math.sin(2 * Math.PI * f * t);
  const sub = Math.sin(2 * Math.PI * (f * 0.5) * t) * 0.5;
  const filtered = sine * 0.6 + sub * 0.4 + saw * 0.15 * Math.exp(-t * 8);
  return filtered * Math.exp(-t * 3.0) * Math.min(1, t * 400) * 0.95;
});

// --- Harmony & Progression ---
// Fmaj7 -> G -> Em7 -> Am7 (uplifting, contemporary SaaS vibe)
const CHORD_PROG = [
  {name: 'Fmaj7', root: 43.65, tones: [174.61, 220.0, 261.63, 329.63]}, // F3, A3, C4, E4
  {name: 'G',     root: 49.00, tones: [196.0,  246.94, 293.66, 392.0]},  // G3, B3, D4, G4
  {name: 'Em7',   root: 41.20, tones: [164.81, 196.0,  246.94, 293.66]}, // E3, G3, B3, D4
  {name: 'Am7',   root: 55.00, tones: [220.0,  261.63, 329.63, 392.0]},  // A3, C4, E4, G4
];

const totalBars = Math.floor(DURATION / (BEAT * 4)); // ~15.5 bars
const DROP_TIME = 2.0; // Beat drops early at 2 seconds
const OUTRO_TIME = 25.5; // Final celebratory hit

// Intro birds
mix(sfx, 0.2, birdWarble(2200, 3000, 0.12), 0.8);
mix(sfx, 0.38, birdWarble(3100, 3700, 0.15), 0.9);
mix(sfx, 1.2, birdWarble(2600, 3200, 0.14), 0.7);

// Main Bar Loop
for (let b = 0; b < totalBars; b++) {
  const barTime = b * BEAT * 4;
  const chordObj = barTime >= OUTRO_TIME ? CHORD_PROG[0] : CHORD_PROG[b % 4];
  const isDrop = barTime >= DROP_TIME && barTime < 28.5;

  // 1. Chords (Electric Piano)
  chordObj.tones.forEach((freq, idx) => {
    // Play on beat 1 and syncopated beat 2.5
    mix(chords, barTime + idx * 0.015, rhodesTine(freq, BEAT * 2), 0.25);
    if (isDrop) {
      mix(chords, barTime + BEAT * 1.5 + idx * 0.012, rhodesTine(freq, BEAT * 2.5), 0.22);
    }
  });

  // 2. Rhythm & Percussion
  for (let e = 0; e < 8; e++) {
    const t = barTime + e * (BEAT / 2);
    if (t >= 28.5) continue;

    // Shakers & Hi-hats
    const shakerGain = isDrop ? (e % 2 === 0 ? 0.22 : 0.14) : 0.1;
    mix(drums, t, modernShaker(shakerGain));
    if (isDrop) {
      mix(drums, t + BEAT / 4, modernShaker(0.08));
    }

    if (!isDrop) continue;

    // Bassline Groove (Bouncy syncopation)
    if ([0, 2, 3, 5, 6].includes(e)) {
      const pitch =
        e === 3 ? chordObj.root * 1.5 :
        e === 5 ? chordObj.root * 1.25 :
        e === 6 ? chordObj.root * 2 :
        chordObj.root;
      mix(bassTrack, t, modernBass(pitch, 0.4), 0.65);
    }

    // Marimba melodic arpeggio
    if ([0, 1, 3, 4, 6].includes(e)) {
      const pat = [0, 2, 1, 3, 2];
      const tone = chordObj.tones[pat[e % pat.length]] * 2;
      mix(melodic, t, bouncyMarimba(tone, 0.5), 0.18);
    }
  }

  // 3. Four-on-the-floor kick & snappy clap on 2 & 4
  for (let beat = 0; beat < 4; beat++) {
    const t = barTime + beat * BEAT;
    if (isDrop && t < 28.5) {
      mix(drums, t, modernKick(), 0.85);
      if (beat === 1 || beat === 3) {
        mix(drums, t, snappyClap(), 0.78);
      }
    }
  }

  // 4. Sparkle Bell Accents on chord turns
  if (isDrop && b % 2 === 0) {
    mix(melodic, barTime, chimeSparkle(chordObj.tones[3] * 3), 0.18);
    mix(melodic, barTime + BEAT * 2, chimeSparkle(chordObj.tones[2] * 3), 0.14);
  }

  // Scene transition whooshes every 4 bars (~7.5s)
  if (b === 4 || b === 8 || b === 12) {
    mix(sfx, barTime - 0.4, whooshSweep(0.4, true), 0.55);
    mix(sfx, barTime, uiPop(1400), 0.4);
    mix(sfx, barTime + 0.05, birdWarble(2800, 3600, 0.12), 0.6);
  }
}

// Drop Buildup at 1.0s to 2.0s
for (let i = 0; i < 4; i++) {
  mix(drums, DROP_TIME - 1.0 + i * 0.25, snappyClap(), 0.35 + i * 0.12);
}
mix(sfx, DROP_TIME - 0.5, whooshSweep(0.5, true), 0.6);
mix(drums, DROP_TIME, modernKick(), 1.0);
mix(sfx, DROP_TIME, uiPop(1600), 0.5);

// Outro Hit at 25.5s
mix(drums, OUTRO_TIME, modernKick(), 1.0);
mix(drums, OUTRO_TIME, snappyClap(), 0.9);
mix(sfx, OUTRO_TIME, whooshSweep(0.6, false), 0.5);
CHORD_PROG[0].tones.forEach((f, i) => {
  mix(chords, OUTRO_TIME + i * 0.02, rhodesTine(f, 3.5), 0.35);
});
mix(melodic, OUTRO_TIME + 0.1, chimeSparkle(1046.5), 0.25); // C6
mix(melodic, OUTRO_TIME + 0.35, chimeSparkle(1318.5), 0.22); // E6
mix(melodic, OUTRO_TIME + 0.6, chimeSparkle(1567.98), 0.18); // G6
mix(sfx, OUTRO_TIME + 0.8, birdWarble(3000, 3800, 0.2), 0.85);

// Dotted-eighth stereo delay on melodic tracks
const delaySamples = Math.floor(BEAT * 0.75 * SR);
for (let i = delaySamples; i < N; i++) {
  melodic[i] += melodic[i - delaySamples] * 0.34;
}

// Final Master Mix with soft saturation
const master = new Float32Array(N);
for (let i = 0; i < N; i++) {
  const t = i / SR;
  const fade = Math.min(1, t / 0.15) * Math.min(1, (DURATION - t) / 1.5);
  const raw =
    drums[i] * 0.95 +
    bassTrack[i] * 0.85 +
    chords[i] * 0.8 +
    melodic[i] * 0.75 +
    sfx[i] * 0.8;
  master[i] = Math.tanh(raw * 0.9) * fade * 0.88;
}

wav('music_modern', master);
console.log('Successfully generated public/audio/music_modern.wav');
