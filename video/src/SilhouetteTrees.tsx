import React from 'react';
import {Dusk} from './theme';

/**
 * Rich Vector Illustrated Forest Elements
 * Matches the reference illustration style:
 * - Painterly vector tree trunks with stylized vertical grain/bark fissures and branch elbows
 * - Layered canopy clusters with shadow clumps and sunny highlight puffs
 * - Conifers with jagged needle boughs and interior tonal facets
 * - Winding clearing dirt path, soft mossy grass clusters, and smooth river boulders
 */

interface TreeProps {
  height?: number;
  width?: number;
  flip?: boolean;
}

/**
 * Foreground Tree Trunk - passes very close to camera as the transition divider between slides.
 * Now significantly wider and closer as requested, with rich stylized vector bark textures,
 * knots, branches, and overhanging leaves.
 */
export const IllustratedForegroundTree: React.FC<TreeProps> = ({
  height = 1080,
  width = 950,
  flip = false,
}) => {
  const transform = flip ? 'scaleX(-1)' : undefined;

  return (
    <svg
      width={width}
      height={height}
      viewBox="0 0 950 1080"
      preserveAspectRatio="none"
      fill="none"
      style={{transform, display: 'block'}}
    >
      <defs>
        {/* Subtle dusk rim lighting gradient on tree edges */}
        <linearGradient id="fgTrunkGrad" x1="0%" y1="0%" x2="100%" y2="0%">
          <stop offset="0%" stopColor={Dusk.trunkHighlight} />
          <stop offset="18%" stopColor={Dusk.trunkMid} />
          <stop offset="65%" stopColor={Dusk.trunkBase} />
          <stop offset="100%" stopColor={Dusk.trunkShadow} />
        </linearGradient>
      </defs>

      {/* 1. Main Massive Tree Trunk */}
      <path
        d="
          M 220 1080
          C 290 920, 240 760, 310 580
          C 350 440, 290 280, 340 120
          C 360 40, 320 0, 350 0
          L 680 0
          C 640 120, 710 260, 670 420
          C 740 580, 680 770, 730 920
          C 760 1000, 720 1050, 790 1080
          Z
        "
        fill="url(#fgTrunkGrad)"
      />

      {/* 2. Stylized Bark Texture & Knot Lines (as seen in the reference illustration) */}
      <g stroke={Dusk.barkLine} strokeWidth="6" strokeLinecap="round" opacity="0.85">
        <path d="M 380 980 C 370 880, 395 810, 385 710" />
        <path d="M 440 1040 C 435 940, 460 850, 450 750" />
        <path d="M 520 960 C 510 880, 530 810, 525 720" />
        <path d="M 600 1020 C 590 910, 615 820, 605 730" />
        {/* Oval tree knot */}
        <ellipse cx="480" cy="540" rx="22" ry="38" stroke={Dusk.barkLine} strokeWidth="6" fill={Dusk.trunkShadow} />
        <path d="M 480 500 C 460 480, 450 430, 470 380" />
        <path d="M 550 560 C 570 490, 560 410, 580 340" />
        <path d="M 400 390 C 420 310, 410 220, 430 140" />
        <path d="M 500 320 C 520 240, 510 160, 530 80" />
      </g>

      {/* 3. Deep Left Elbow Branch */}
      <path
        d="
          M 330 520
          C 250 480, 180 510, 90 460
          C 30 420, -30 440, -100 400
          C -40 430, 20 450, 80 480
          C 160 520, 230 535, 320 570
          Z
        "
        fill={Dusk.trunkMid}
      />
      {/* Branch highlight rim */}
      <path
        d="M -100 400 C -30 440, 30 420, 90 460 C 180 510, 250 480, 330 520"
        stroke={Dusk.foliageWarm}
        strokeWidth="6"
        fill="none"
      />

      {/* 4. High Sweeping Right Limb */}
      <path
        d="
          M 670 360
          C 760 310, 850 330, 960 290
          C 1050 260, 1140 280, 1220 240
          C 1130 290, 1040 310, 950 340
          C 840 380, 750 370, 660 410
          Z
        "
        fill={Dusk.trunkMid}
      />
      <path
        d="M 670 360 C 760 310, 850 330, 960 290 C 1050 260, 1140 280, 1220 240"
        stroke={Dusk.foliageWarm}
        strokeWidth="6"
        fill="none"
      />

      {/* 5. Clustered Foliage Puffs along branches (Illustrative rounded shapes) */}
      {/* Left cluster */}
      <g>
        <ellipse cx="60" cy="420" rx="95" ry="65" fill={Dusk.foliageDark} />
        <ellipse cx="40" cy="405" rx="80" ry="55" fill={Dusk.foliageMid} />
        <ellipse cx="25" cy="390" rx="60" ry="40" fill={Dusk.foliageLight} />
        <ellipse cx="10" cy="380" rx="40" ry="25" fill={Dusk.foliageWarm} opacity="0.8" />
      </g>

      {/* Right cluster */}
      <g>
        <ellipse cx="880" cy="270" rx="110" ry="75" fill={Dusk.foliageDark} />
        <ellipse cx="860" cy="255" rx="90" ry="60" fill={Dusk.foliageMid} />
        <ellipse cx="840" cy="240" rx="70" ry="45" fill={Dusk.foliageLight} />
        <ellipse cx="825" cy="230" rx="50" ry="30" fill={Dusk.foliageWarm} opacity="0.8" />
      </g>
    </svg>
  );
};

/**
 * Illustrated Deciduous Oak (Midground)
 * Multi-lobed canopy with shaded underside, mid-tones, and sunny sunset highlights.
 */
export const IllustratedOak: React.FC<TreeProps> = ({
  height = 800,
  width = 500,
  flip = false,
}) => {
  const transform = flip ? 'scaleX(-1)' : undefined;

  return (
    <svg
      width={width}
      height={height}
      viewBox="0 0 500 800"
      preserveAspectRatio="none"
      fill="none"
      style={{transform, display: 'block'}}
    >
      {/* Trunk and forked limbs */}
      <path
        d="
          M 215 800 C 230 650, 215 520, 235 380
          C 180 340, 110 320, 40 280
          C 105 310, 170 330, 225 360
          C 245 310, 275 280, 320 240
          C 370 200, 430 190, 480 160
          C 430 200, 370 220, 320 260
          C 270 300, 265 440, 285 620
          L 295 800 Z
        "
        fill={Dusk.trunkBase}
      />
      {/* Trunk bark highlights */}
      <path d="M 245 680 C 240 580, 255 500, 250 420" stroke={Dusk.trunkMid} strokeWidth="5" />

      {/* Canopy Clumps: Shadow Base */}
      <circle cx="250" cy="230" r="145" fill={Dusk.foliageDark} />
      <circle cx="150" cy="270" r="115" fill={Dusk.foliageDark} />
      <circle cx="350" cy="260" r="120" fill={Dusk.foliageDark} />

      {/* Canopy Clumps: Midtone Flesh */}
      <circle cx="245" cy="215" r="130" fill={Dusk.foliageMid} />
      <circle cx="145" cy="255" r="100" fill={Dusk.foliageMid} />
      <circle cx="345" cy="245" r="105" fill={Dusk.foliageMid} />

      {/* Canopy Clumps: Bright Sunset Highlight Caps */}
      <circle cx="235" cy="180" r="100" fill={Dusk.foliageLight} />
      <circle cx="135" cy="225" r="75" fill={Dusk.foliageLight} />
      <circle cx="335" cy="215" r="80" fill={Dusk.foliageLight} />

      {/* Golden Dusk Rim Light Tips */}
      <ellipse cx="230" cy="140" rx="60" ry="35" fill={Dusk.foliageWarm} opacity="0.85" />
      <ellipse cx="130" cy="190" rx="45" ry="25" fill={Dusk.foliageWarm} opacity="0.85" />
      <ellipse cx="330" cy="180" rx="50" ry="28" fill={Dusk.foliageWarm} opacity="0.85" />
    </svg>
  );
};

/**
 * Illustrated Evergreen Pine / Conifer (with shaded geometric needle tiers)
 */
export const IllustratedPine: React.FC<TreeProps> = ({
  height = 700,
  width = 280,
  flip = false,
}) => {
  const transform = flip ? 'scaleX(-1)' : undefined;

  return (
    <svg
      width={width}
      height={height}
      viewBox="0 0 280 700"
      preserveAspectRatio="none"
      fill="none"
      style={{transform, display: 'block'}}
    >
      {/* Bare trunk base */}
      <rect x="132" y="420" width="16" height="280" fill={Dusk.trunkBase} />

      {/* Left shadow side of conifer */}
      <path
        d="
          M 140 20
          L 155 90 L 145 90 L 170 170 L 150 170 L 185 260 L 160 260
          L 205 370 L 175 370 L 230 490 L 140 490
          Z
        "
        fill={Dusk.pineLight}
      />
      {/* Right sunny side of conifer */}
      <path
        d="
          M 140 20
          L 125 90 L 135 90 L 110 170 L 130 170 L 95 260 L 120 260
          L 75 370 L 105 370 L 50 490 L 140 490
          Z
        "
        fill={Dusk.pineDark}
      />
      {/* Interior facet highlights */}
      <path
        d="
          M 140 20 L 145 90 L 140 170 L 148 260 L 140 370 L 145 490
        "
        stroke={Dusk.pineHighlight}
        strokeWidth="3"
      />
    </svg>
  );
};

/**
 * Illustrated Forest Clearing Floor
 * Features the winding dirt path, grass puffs, and mossy river boulders seen in the reference art.
 */
export const IllustratedGround: React.FC<{width?: number}> = ({width = 1920}) => {
  return (
    <svg
      width={width}
      height={320}
      viewBox={`0 0 ${width} 320`}
      preserveAspectRatio="none"
      fill="none"
      style={{display: 'block', position: 'absolute', bottom: 0, left: 0}}
    >
      {/* 1. Base Meadow Floor */}
      <rect x="0" y="80" width={width} height="240" fill={Dusk.groundDeep} />
      <path
        d={`M 0 80 Q ${width * 0.25} 50 ${width * 0.5} 80 T ${width} 75 L ${width} 120 L 0 120 Z`}
        fill={Dusk.groundMid}
      />

      {/* 2. Winding Sunset Path (golden peach-amber to dusky rose) */}
      <path
        d={`
          M ${width * 0.35} 320
          C ${width * 0.42} 240, ${width * 0.38} 180, ${width * 0.45} 120
          C ${width * 0.48} 90, ${width * 0.52} 75, ${width * 0.5} 60
          L ${width * 0.52} 60
          C ${width * 0.54} 75, ${width * 0.52} 90, ${width * 0.56} 120
          C ${width * 0.62} 180, ${width * 0.72} 240, ${width * 0.78} 320
          Z
        `}
        fill={Dusk.pathColor}
        opacity="0.8"
      />
      {/* Path highlight glow */}
      <path
        d={`
          M ${width * 0.44} 320
          C ${width * 0.48} 250, ${width * 0.44} 190, ${width * 0.48} 130
          L ${width * 0.52} 130
          C ${width * 0.55} 190, ${width * 0.62} 250, ${width * 0.68} 320
          Z
        `}
        fill={Dusk.pathGlow}
        opacity="0.65"
      />

      {/* 3. Stylized River Boulders (Right side) */}
      <ellipse cx={width * 0.75} cy="230" rx="90" ry="55" fill={Dusk.rockShadow} />
      <ellipse cx={width * 0.74} cy="222" rx="82" ry="48" fill={Dusk.rockBody} />
      <ellipse cx={width * 0.72} cy="210" rx="65" ry="35" fill={Dusk.rockHighlight} />

      <ellipse cx={width * 0.82} cy="250" rx="55" ry="32" fill={Dusk.rockShadow} />
      <ellipse cx={width * 0.81} cy="244" rx="48" ry="28" fill={Dusk.rockBody} />

      {/* 4. Bush & Foliage Clumps along edges */}
      <ellipse cx={width * 0.12} cy="180" rx="75" ry="45" fill={Dusk.foliageDark} />
      <ellipse cx={width * 0.11} cy="172" rx="65" ry="38" fill={Dusk.foliageMid} />
      <ellipse cx={width * 0.10} cy="165" rx="50" ry="28" fill={Dusk.foliageLight} />

      <ellipse cx={width * 0.9} cy="200" rx="85" ry="50" fill={Dusk.foliageDark} />
      <ellipse cx={width * 0.89} cy="190" rx="72" ry="42" fill={Dusk.foliageMid} />
      <ellipse cx={width * 0.88} cy="182" rx="55" ry="30" fill={Dusk.foliageLight} />
    </svg>
  );
};
