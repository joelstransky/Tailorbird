# Tailorbird Project Agent Rules & Guidelines

## 1. ABSOLUTE PII RESTRICTION (CRITICAL RULE)
> **NEVER, EVER, EVER USE THE USER'S PERSONALLY IDENTIFIABLE INFORMATION (PII) IN ANY CONTENT ANYWHERE IN THIS PRODUCT OR REPOSITORY.**
> 
> - **Prohibited Information**: Real names (e.g., "Joel", "Stransky"), personal/home addresses, phone numbers (e.g., 915-474-2746, 808-895-7171, or any variation), personal email addresses, personal social profiles, or personal portfolio URLs.
> - **Zero Tolerance Scope**: This rule applies without exception to UI mockups, marketing copy, demo data, test fixtures, seeded sample databases, documentation screenshots, video compositions, and scripts.

## 2. BIRD-THEMED CONTENT MOTIF
All placeholder text, sample profiles, demo job applications, company names, and promotional materials must strictly use the **bird world motif**:
- **Candidate Names**: *Robin Featherstone*, *Finley Sparrow*, *Wren Macaw*, *Peregrine Swift*
- **Companies**: *Wren Industries*, *Owl Analytics*, *Finch & Sparrow*, *Peckwell Labs*, *Falcon Dynamics*, *Heron Logistics*, *Beakwell & Co.*
- **Job Titles**: *Senior Nest Architect*, *Night Shift Data Scientist*, *Seed Procurement Lead*, *Canopy Operations Manager*, *High Velocity Flight Engineer*
- **Contact Details**:
  - Email: `robin@maplegrove.dev`, `nest@aviary.example`
  - Phone: `(555) 247-3637` (555-BIRD-NEST) or any fictional `(555) 01XX` numbers
  - URLs: `aviary.network/in/robin-featherstone`, `maplegrove.dev`
  - Locations: *Maple Grove Canopy*, *Pacific NW Treetops*, *Alpine Crest Reserve*

## 3. REMOTION VIDEO PROJECT STANDARDS
- **Framing & Parallax**: 360-degree continuous panoramic rotation with infinite looping parallax layers (Farground 0.2x, Midground 0.5x).
- **Motion Continuity**: Parallax background motion must maintain a continuous gentle drift and never come to an abrupt dead stop.
- **Divider Silhouette**: Massive organic dark silhouette tree transitions between scenes, moving across in one continuous motion without lingering over slide text.
- **Broadcast Motion Aesthetics**: Punchy typography slams and 2.5D perspective views without generic rounded web-style containers or eyebrows.
- **Audio Integrity**: Upbeat synthesized music track in `public/audio/music.wav` preserved intact.

## 4. RELEASE DOWNLOAD LINKS MAINTENANCE (NEW BUILDS)
Whenever a new build number or release version tag is created (e.g., `v0.1.1`, `v0.2.0`), the agent MUST update the OS download links in `docs/index.html`:
- **Windows**: `https://github.com/joelstransky/Tailorbird/releases/download/<TAG>/Tailorbird-Setup-<TAG>.exe`
- **macOS**: `https://github.com/joelstransky/Tailorbird/releases/download/<TAG>/Tailorbird-macOS.dmg`
- **Linux**: `https://github.com/joelstransky/Tailorbird/releases/download/<TAG>/tailorbird-linux-x86_64.tar.gz`
- **Release notes**: `https://github.com/joelstransky/Tailorbird/releases/tag/<TAG>`
- **Button Presentation**:
  - Keep buttons strictly minimal: only the platform icon, the name of the OS (`Windows`, `macOS`, `Linux`), and the download action icon.
  - Do NOT display file names, file extensions (e.g., avoid `.exe`, `.dmg`, `.tar.gz`), architecture strings, version tags, or subtitles inside the buttons.
  - Preserve standard platform SVG logos and the active OS detection script.

