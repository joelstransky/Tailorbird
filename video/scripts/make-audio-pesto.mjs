// Synthesizes fresh Pesto Mint acoustic soundtrack into public/audio/music_pesto.wav
// 120bpm, C - G - Am - F progression.
// Bright morning canopy feel:
// - Cheerful acoustic guitar/ukulele plucks (Karplus-Strong)
// - Bird-whistle morning chirps
// - Wooden marimba / kalimba bouncing arpeggios
// - Glockenspiel & chime sparkles
// - Warm round walking acoustic bass
// - Snappy live acoustic percussion (kick, wooden rimshot/clap, tambourine, shaker)
// - Transition riser & crash swells

import {mkdirSync, writeFileSync} from 'node:fs';

const SR = 44100;
const DURATION = 30;
const BEAT = 0.5; // 120bpm (15 frames/beat at 30fps)
mkdirSync('public/audio', {recursive: true});

let seed = 42427;
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
const melodic = new Float32Array(N); // with dotted-eighth echo
const dry = new Float32Array(N);     // bass, strum, pad
const sparkles = new Float32Array(N); // high chimes, bird chirps

function mix(bus, startSec, arr, gain = 1) {
  const s0 = Math.floor(startSec * SR);
  for (let i = 0; i < arr.length && s0 + i < N; i++) {
    bus[s0 + i] += arr[i] * gain;
  }
}

const gen = (dur, fn) => Float32Array.from({length: Math.floor(dur * SR)}, (_, i) => fn(i / SR, i));

// --- Sound Generators ---

// Cheerful bird chirp whistle
const birdChirp = (startF, endF, dur = 0.14) => gen(dur, (t) => {
  const f = startF + (endF - startF) * (t / dur) + Math.sin(2 * Math.PI * 35 * t) * 60;
  const env = Math.sin(Math.PI * (t / dur)) ** 1.8;
  return Math.sin(2 * Math.PI * f * t) * env * 0.45;
});

// Acoustic kick with warm punch
const kick = () => gen(0.32, (t) => {
  const f = 45 + 110 * Math.exp(-t * 32);
  return Math.sin(2 * Math.PI * f * t) * Math.exp(-t * 11) * 0.95;
});

// Crisp wooden rimshot & clap
const rimClap = () => gen(0.22, (t) => {
  const rim = Math.sin(2 * Math.PI * 880 * t) * Math.exp(-t * 45) * 0.5;
  const burst = [0, 0.01, 0.022].reduce((a, o) => a + (t > o ? Math.exp(-(t - o) * 80) : 0), 0);
  return (rim + rnd() * burst * 0.45 + rnd() * Math.exp(-t * 22) * 0.25) * 0.85;
});

// Tambourine jingle
const tambourine = () => gen(0.18, (t) => {
  const jingle = (Math.sin(2 * Math.PI * 5200 * t) + Math.sin(2 * Math.PI * 7400 * t)) * 0.3;
  return (rnd() * 0.4 + jingle) * Math.exp(-t * 28) * 0.35;
});

// Shaker
const shaker = (acc) => {
  let prev = 0;
  return gen(0.06, (t) => {
    const n = rnd();
    const hp = n - prev;
    prev = n;
    return hp * Math.exp(-t * 60) * acc;
  });
};

const snareRiser = (dur) => {
  let lp = 0;
  return gen(dur, (t) => {
    lp += (0.04 + 0.65 * (t / dur)) * (rnd() - lp);
    return lp * (t / dur) ** 2 * 1.6;
  });
};

const crashCymbal = () => {
  let prev = 0;
  return gen(2.4, (t) => {
    const n = rnd();
    const hp = n - prev;
    prev = n;
    return hp * Math.exp(-t * 2.4) * 0.4;
  });
};

// Karplus-Strong acoustic guitar / ukulele pluck
function pluck(freq, dur, bright = 0.6) {
  const len = Math.max(2, Math.round(SR / freq));
  const line = Float32Array.from({length: len}, () => rnd() * bright);
  let idx = 0;
  return gen(dur, (t) => {
    const a = line[idx];
    const b = line[(idx + 1) % len];
    const v = (a + b) * 0.5 * 0.997;
    line[idx] = v;
    idx = (idx + 1) % len;
    return a * Math.exp(-t * 0.75);
  });
}

// Bouncy marimba/kalimba
const marimba = (f, dur = 0.75) => gen(dur, (t) => {
  const fundamental = Math.sin(2 * Math.PI * f * t) * Math.exp(-t * 6);
  const harmonic2 = Math.sin(2 * Math.PI * f * 3.8 * t) * Math.exp(-t * 26) * 0.4;
  const harmonic3 = Math.sin(2 * Math.PI * f * 9.2 * t) * Math.exp(-t * 60) * 0.2;
  return (fundamental + harmonic2 + harmonic3) * Math.min(1, t * 500) * 0.8;
});

// Glockenspiel bell sparkle
const glock = (f) => gen(1.8, (t) => {
  return (
    Math.sin(2 * Math.PI * f * t) +
    Math.sin(2 * Math.PI * f * 2.76 * t) * 0.45 +
    Math.sin(2 * Math.PI * f * 5.4 * t) * 0.18
  ) * Math.exp(-t * 2.8) * 0.5;
});

// Warm round acoustic bass
const bass = (f, dur) => gen(dur, (t) => {
  const body = Math.sin(2 * Math.PI * f * t) * Math.exp(-t * 3.2);
  const sub = Math.sin(2 * Math.PI * (f * 0.5) * t) * Math.exp(-t * 2.8) * 0.25;
  const attack = Math.sin(2 * Math.PI * f * 2 * t) * Math.exp(-t * 14) * 0.35;
  return (body + sub + attack) * Math.min(1, t * 350) * 0.95;
});

// Soft pastel mint string pad
const mintPad = (f, dur) => gen(dur, (t) => {
  const env = Math.min(1, t / 0.45) * Math.min(1, Math.max(0, (dur - t) / 0.55));
  let s = 0;
  for (const d of [-0.003, 0, 0.003]) {
    s += Math.sin(2 * Math.PI * f * (1 + d) * t) * 0.3 + ((f * (1 + d) * t) % 1 - 0.5) * 0.15;
  }
  return s * env * 0.08;
});

// Soft French horn / brass chime for outro
const softHorn = (f, dur) => {
  let lp = 0;
  return gen(dur, (t) => {
    const env = Math.min(1, t / 0.15) * Math.exp(-t * 0.9);
    const saw = ((f * t) % 1) * 2 - 1;
    lp += 0.08 * (saw - lp);
    return lp * env * 0.45;
  });
};

// --- Composition Structure ---
const CHORDS = {
  C: {root: 65.41, tones: [261.63, 329.63, 392.0, 523.25]},
  G: {root: 49.0, tones: [196.0, 246.94, 293.66, 392.0]},
  Am: {root: 55.0, tones: [220.0, 261.63, 329.63, 440.0]},
  F: {root: 43.65, tones: [174.61, 220.0, 261.63, 349.23]},
};
const PROG = ['C', 'G', 'Am', 'F'];
const DROP = 3.0; // 3.0s: main groove launches
const END_HIT = 26.0; // 26.0s: resolution chord and CTA

// Bird chirps in the morning intro
mix(sparkles, 0.2, birdChirp(2100, 2800, 0.12), 0.7);
mix(sparkles, 0.38, birdChirp(2900, 3400, 0.14), 0.85);
mix(sparkles, 1.4, birdChirp(2400, 3100, 0.15), 0.75);
mix(sparkles, 1.6, birdChirp(3200, 2600, 0.16), 0.8);

// Bars loop
const bars = DURATION / (BEAT * 4);
for (let b = 0; b < bars; b++) {
  const t0 = b * BEAT * 4;
  const chordName = t0 >= END_HIT ? 'C' : PROG[b % 4];
  const ch = CHORDS[chordName];

  for (let e = 0; e < 8; e++) {
    const t = t0 + e * (BEAT / 2);
    const main = t >= DROP && t < 28;

    // Shakers & Tambourine
    if (t < 28) {
      mix(drums, t, shaker(main ? (e % 2 ? 0.22 : 0.14) : 0.12));
      if (main) {
        mix(drums, t + BEAT / 4, shaker(0.09));
        if (e === 2 || e === 6) mix(drums, t, tambourine(), 0.55);
      }
    }

    if (!main) continue;

    // Bouncy Walking Bass
    if ([0, 2, 4, 6, 7].includes(e)) {
      const pitch = e === 4 ? ch.root * 1.5 : e === 6 ? ch.root * 1.25 : e === 7 ? ch.root * 2 : ch.root;
      mix(dry, t, bass(pitch, 0.42), 0.6);
    }

    // Upbeat strumming
    if ([0, 3, 4, 6, 7].includes(e)) {
      const down = e % 2 === 0;
      const tones = down ? ch.tones : [...ch.tones].reverse();
      tones.forEach((f, i) => mix(dry, t + i * 0.011, pluck(f, 0.95), down ? 0.22 : 0.15));
    }

    // Marimba arpeggio (bouncing pattern)
    if (![3, 7].includes(e)) {
      const pat = [0, 2, 1, 3, 2, 3, 1, 2];
      mix(melodic, t, marimba(ch.tones[pat[e]] * 2), 0.18);
    }
  }

  // Four-on-the-floor kick & clap
  for (let beat = 0; beat < 4; beat++) {
    const t = t0 + beat * BEAT;
    if (t >= DROP && t < 28) {
      mix(drums, t, kick(), 0.85);
      if (beat % 2 === 1) mix(drums, t, rimClap(), 0.75);
    }
  }

  // Mint pad on chords
  if (t0 + 2 > DROP && t0 < 28) {
    const pt = Math.max(t0, DROP);
    ch.tones.slice(0, 3).forEach((f) => mix(dry, pt, mintPad(f, BEAT * 4 + 0.3), 1));
  }

  // Glockenspiel sparkles
  if (t0 >= DROP - 0.01 && t0 < 28) {
    mix(sparkles, t0, glock(ch.tones[2] * 4), 0.16);
    mix(sparkles, t0 + BEAT * 2, glock(ch.tones[1] * 4), 0.12);
    // Add subtle bird chirp on every 4th bar
    if (b % 4 === 2) {
      mix(sparkles, t0 + BEAT * 3, birdChirp(2600, 3300, 0.14), 0.5);
    }
  }

  // Intro (t0 < DROP): gentle plucked intro with soft bass & marimba
  if (t0 < DROP) {
    for (const beat of [0, 2]) {
      const t = t0 + beat * BEAT;
      if (t < DROP) {
        mix(dry, t, bass(ch.root * 2, 0.5), 0.5);
        mix(melodic, t + BEAT / 2, marimba(ch.tones[1] * 2, 0.6), 0.15);
      }
    }
    ch.tones.forEach((f, i) => mix(dry, t0 + i * 0.014, pluck(f, 1.1), 0.16));
  }
}

// Build-up into the Drop (2.0s to 3.0s)
mix(drums, DROP - 1.0, snareRiser(1.0), 0.6);
for (let i = 0; i < 6; i++) mix(drums, DROP - 1.0 + i * 0.25, rimClap(), 0.3 + i * 0.1);
for (let i = 0; i < 4; i++) mix(drums, DROP - 0.5 + i * 0.125, rimClap(), 0.55 + i * 0.12);
mix(drums, DROP, crashCymbal(), 1.0);

// Resolving Ending at 26.0s
mix(drums, END_HIT, crashCymbal(), 0.75);
CHORDS.C.tones.forEach((f, i) => mix(dry, END_HIT + i * 0.02, pluck(f, 2.8), 0.3));
mix(sparkles, END_HIT + 0.08, glock(1046.5), 0.22); // C6
mix(sparkles, END_HIT + 0.32, glock(1318.5), 0.18); // E6
mix(sparkles, END_HIT + 0.55, glock(1567.98), 0.15); // G6
mix(sparkles, END_HIT + 0.75, birdChirp(2800, 3600, 0.2), 0.7); // Happy goodbye chirp
mix(dry, END_HIT, softHorn(261.63, 2.6), 0.45);

// Dotted-eighth stereo-like delay echo on melodic & sparkles bus
const delay = Math.floor(BEAT * 0.75 * SR);
for (let i = delay; i < N; i++) {
  melodic[i] += melodic[i - delay] * 0.36;
  sparkles[i] += sparkles[i - delay] * 0.28;
}

// Master output mix with gentle warmth and soft limiter
const out = new Float32Array(N);
for (let i = 0; i < N; i++) {
  const t = i / SR;
  const fade = Math.min(1, t / 0.25) * Math.min(1, (DURATION - t) / 1.6);
  const raw = drums[i] * 0.9 + melodic[i] + dry[i] * 0.85 + sparkles[i] * 0.85;
  out[i] = Math.tanh(raw * 0.92) * fade * 0.86;
}

wav('music_pesto', out);
// Also update music.wav so default Launch uses this fresh soundtrack too
wav('music', out);

console.log('Successfully generated public/audio/music_pesto.wav and public/audio/music.wav');
