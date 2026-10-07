// Synthesizes placeholder audio into public/audio (click, whoosh, music). Swap in real tracks any time.
// Music: 120bpm, C-G-Am-F. Plucked ukulele (Karplus-Strong), kalimba/marimba arps with echo,
// glockenspiel sparkle, soft string pad, warm bass, light kit. One very soft horn note at the end.
import {mkdirSync, writeFileSync} from 'node:fs';

const SR = 44100;
const DURATION = 30;
const BEAT = 0.5; // 120bpm: matches 15 frames/beat in src/timing.ts
mkdirSync('public/audio', {recursive: true});

let seed = 1337;
const rnd = () => ((seed = (seed * 1664525 + 1013904223) >>> 0) / 4294967296) * 2 - 1;

function wav(name, samples) {
  const buf = Buffer.alloc(44 + samples.length * 2);
  buf.write('RIFF', 0); buf.writeUInt32LE(36 + samples.length * 2, 4); buf.write('WAVEfmt ', 8);
  buf.writeUInt32LE(16, 16); buf.writeUInt16LE(1, 20); buf.writeUInt16LE(1, 22);
  buf.writeUInt32LE(SR, 24); buf.writeUInt32LE(SR * 2, 28); buf.writeUInt16LE(2, 32); buf.writeUInt16LE(16, 34);
  buf.write('data', 36); buf.writeUInt32LE(samples.length * 2, 40);
  for (let i = 0; i < samples.length; i++) buf.writeInt16LE(Math.max(-1, Math.min(1, samples[i])) * 32767, 44 + i * 2);
  writeFileSync(`public/audio/${name}.wav`, buf);
}

/* ---- SFX ---- */
wav('click', Float32Array.from({length: SR * 0.06}, (_, i) => {
  const t = i / SR;
  return rnd() * Math.exp(-t * 90) * 0.6 + Math.sin(2 * Math.PI * 1800 * t) * Math.exp(-t * 70) * 0.4;
}));
{
  let lp = 0;
  wav('whoosh', Float32Array.from({length: SR * 0.5}, (_, i) => {
    const t = i / SR;
    const env = Math.sin(Math.PI * Math.min(1, t / 0.5)) ** 2;
    lp += (0.02 + 0.5 * (t / 0.5)) * (rnd() - lp);
    return lp * env * 1.6;
  }));
}

/* ---- Music engine ---- */
const N = SR * DURATION;
const drums = new Float32Array(N);
const melodic = new Float32Array(N); // gets an echo
const dry = new Float32Array(N); // bass, pad, strum

function mix(bus, startSec, arr, gain = 1) {
  const s0 = Math.floor(startSec * SR);
  for (let i = 0; i < arr.length && s0 + i < N; i++) bus[s0 + i] += arr[i] * gain;
}
const gen = (dur, fn) => Float32Array.from({length: Math.floor(dur * SR)}, (_, i) => fn(i / SR, i));

const kick = () => gen(0.35, (t) => Math.sin(2 * Math.PI * (48 + 90 * Math.exp(-t * 28)) * t) * Math.exp(-t * 9) * 0.9);
const clap = () => gen(0.25, (t) => {
  const bursts = [0, 0.012, 0.024].reduce((a, o) => a + (t > o ? Math.exp(-(t - o) * 70) : 0), 0);
  return rnd() * (bursts * 0.35 + Math.exp(-t * 18) * 0.25);
});
const shaker = (acc) => {
  let prev = 0;
  return gen(0.07, (t) => { const n = rnd(); const hp = n - prev; prev = n; return hp * Math.exp(-t * 55) * acc; });
};
const snareHit = (v) => gen(0.18, (t) => (rnd() * 0.5 + Math.sin(2 * Math.PI * 190 * t) * 0.3) * Math.exp(-t * 22) * v);
const crash = () => {
  let prev = 0;
  return gen(2.2, (t) => { const n = rnd(); const hp = n - prev; prev = n; return hp * Math.exp(-t * 2.2) * 0.35; });
};
const riser = (dur) => {
  let lp = 0;
  return gen(dur, (t) => { lp += (0.05 + 0.6 * (t / dur)) * (rnd() - lp); return lp * (t / dur) ** 2 * 1.5; });
};

// Karplus-Strong plucked string: ukulele/nylon-guitar flavour
function pluck(freq, dur, bright = 0.5) {
  const len = Math.max(2, Math.round(SR / freq));
  const line = Float32Array.from({length: len}, () => rnd() * bright);
  let idx = 0;
  return gen(dur, (t) => {
    const a = line[idx], b = line[(idx + 1) % len];
    const v = (a + b) * 0.5 * 0.9965;
    line[idx] = v; idx = (idx + 1) % len;
    return a * Math.exp(-t * 0.8);
  });
}
// Kalimba/marimba: sine + fast-decaying overtone
const mallet = (f, dur = 0.9) => gen(dur, (t) =>
  (Math.sin(2 * Math.PI * f * t) * Math.exp(-t * 5) + Math.sin(2 * Math.PI * f * 4 * t) * Math.exp(-t * 22) * 0.35) * Math.min(1, t * 400));
// Glockenspiel: bright inharmonic partials
const glock = (f) => gen(1.6, (t) =>
  (Math.sin(2 * Math.PI * f * t) + Math.sin(2 * Math.PI * f * 2.76 * t) * 0.4 + Math.sin(2 * Math.PI * f * 5.4 * t) * 0.15) * Math.exp(-t * 3.2) * 0.5);
// Warm bass
const bass = (f, dur) => gen(dur, (t) =>
  (Math.sin(2 * Math.PI * f * t) + Math.sin(2 * Math.PI * f * 2 * t) * 0.3) * Math.exp(-t * 3.5) * Math.min(1, t * 300));
// Soft string pad (detuned, mellow)
const pad = (f, dur) => gen(dur, (t) => {
  const env = Math.min(1, t / 0.5) * Math.min(1, Math.max(0, (dur - t) / 0.6));
  let s = 0;
  for (const d of [-0.004, 0, 0.004]) {
    const ph = (f * (1 + d) * t) % 1;
    s += (ph * 2 - 1) * 0.5 + Math.sin(2 * Math.PI * f * (1 + d) * t) * 0.5;
  }
  return s * env * 0.07;
});
// Very soft muted horn (used once)
const horn = (f, dur) => {
  let lp = 0;
  return gen(dur, (t) => {
    const env = Math.min(1, t / 0.12) * Math.exp(-t * 1.1);
    const saw = ((f * t) % 1) * 2 - 1;
    lp += 0.07 * (saw - lp);
    return lp * env * 0.5;
  });
};

const CHORDS = {
  C: {root: 65.41, tones: [261.63, 329.63, 392.0, 523.25]},
  G: {root: 49.0, tones: [196.0, 246.94, 293.66, 392.0]},
  Am: {root: 55.0, tones: [220.0, 261.63, 329.63, 440.0]},
  F: {root: 43.65, tones: [174.61, 220.0, 261.63, 349.23]},
};
const PROG = ['C', 'G', 'Am', 'F'];
const DROP = 3.0; // seconds: full band enters (the "Meet Tailorbird" cut)
const END_HIT = 26.0; // CTA: resolve to C

// Kit
const bars = DURATION / (BEAT * 4);
for (let b = 0; b < bars; b++) {
  const t0 = b * BEAT * 4;
  const chordName = t0 >= END_HIT ? 'C' : PROG[b % 4];
  const ch = CHORDS[chordName];

  for (let e = 0; e < 8; e++) {
    const t = t0 + e * (BEAT / 2);
    const main = t >= DROP && t < 28;

    // Shaker: soft 8ths in the intro, 16ths with accents after the drop
    if (t < 28) {
      mix(drums, t, shaker(main ? (e % 2 ? 0.2 : 0.12) : 0.1));
      if (main) mix(drums, t + BEAT / 4, shaker(0.08));
    }
    if (!main) continue;

    // Bass: driving 8ths
    if ([0, 2, 4, 6, 7].includes(e)) mix(dry, t, bass(e === 4 ? ch.root * 2 : e === 7 ? ch.root * 1.5 : ch.root, 0.4), 0.55);

    // Ukulele strums, syncopated down/up
    if ([0, 3, 4, 6, 7].includes(e)) {
      const down = e % 2 === 0;
      const tones = down ? ch.tones : [...ch.tones].reverse();
      tones.forEach((f, i) => mix(dry, t + i * 0.012, pluck(f, 0.9), down ? 0.2 : 0.13));
    }

    // Kalimba/marimba arpeggio, rests on some off-beats for air
    if (![3, 7].includes(e)) {
      const pat = [0, 1, 2, 3, 2, 1, 2, 3];
      mix(melodic, t, mallet(ch.tones[pat[e]] * 2), 0.17);
    }
  }
  for (let beat = 0; beat < 4; beat++) {
    const t = t0 + beat * BEAT;
    if (t >= DROP && t < 28) {
      mix(drums, t, kick(), 0.8);
      if (beat % 2 === 1) mix(drums, t, clap(), 0.7);
    }
  }
  // Pad + glockenspiel on beat 1 after the drop
  if (t0 + 2 > DROP) {
    const pt = Math.max(t0, DROP);
    ch.tones.slice(0, 3).forEach((f) => mix(dry, pt, pad(f, BEAT * 4 + 0.3), 1));
  }
  if (t0 >= DROP - 0.01 && t0 < 28) {
    mix(melodic, t0, glock(ch.tones[2] * 4), 0.14);
    mix(melodic, t0 + BEAT * 2, glock(ch.tones[1] * 4), 0.1);
  }
  // Intro: gentle plucked bass notes
  if (t0 < DROP) {
    for (const beat of [0, 2]) {
      const t = t0 + beat * BEAT;
      if (t < DROP) { mix(dry, t, bass(ch.root * 2, 0.5), 0.5); mix(melodic, t + BEAT / 2, mallet(ch.tones[1] * 2, 0.6), 0.12); }
    }
    ch.tones.forEach((f, i) => mix(dry, t0 + i * 0.015, pluck(f, 1.0), 0.14));
  }
}

// Build into the drop: snare roll (8ths then 16ths), riser, crash on the cut
mix(drums, DROP - 1.0, riser(1.0), 0.5);
for (let i = 0; i < 6; i++) mix(drums, DROP - 1.0 + i * 0.25, snareHit(0.25 + i * 0.1));
for (let i = 0; i < 4; i++) mix(drums, DROP - 0.5 + i * 0.125, snareHit(0.5 + i * 0.12));
mix(drums, DROP, crash(), 1);

// Ending: final resolved chord, ukulele + glock + one soft horn, then ring out
mix(drums, END_HIT, crash(), 0.7);
CHORDS.C.tones.forEach((f, i) => mix(dry, END_HIT + i * 0.02, pluck(f, 2.5), 0.28));
mix(melodic, END_HIT + 0.1, glock(1046.5), 0.2);
mix(melodic, END_HIT + 0.35, glock(1318.5), 0.16);
mix(dry, END_HIT, horn(261.63, 2.5), 0.5); // the only brass: very soft, once

// Echo (dotted 8th) on the melodic bus
const delay = Math.floor(BEAT * 0.75 * SR);
for (let i = delay; i < N; i++) melodic[i] += melodic[i - delay] * 0.38;

const out = new Float32Array(N);
for (let i = 0; i < N; i++) {
  const t = i / SR;
  const fade = Math.min(1, t / 0.3) * Math.min(1, (DURATION - t) / 1.8);
  out[i] = Math.tanh((drums[i] * 0.9 + melodic[i] + dry[i] * 0.85) * 0.9) * fade * 0.85;
}
wav('music', out);

console.log('Wrote public/audio/{click,whoosh,music}.wav');
