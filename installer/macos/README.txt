================================================================================
                    TAILORBIRD — macOS INSTALLATION & HELP
================================================================================

Thank you for trying Tailorbird!

We apologize for any inconvenience caused by Apple Gatekeeper when launching 
Tailorbird for the first time. Because Tailorbird is an independent open-source 
project and is not yet signed with a paid Apple Developer certificate ($99/yr), 
macOS will display a warning dialog saying:

    "Tailorbird" cannot be opened because Apple cannot check it for malicious software.
    or
    "Tailorbird" is damaged and can't be opened. You should move it to the Trash.

Rest assured, Tailorbird is 100% open-source, completely local-first, contains NO 
trackers or telemetry, and all source code is publicly inspectable at:
https://github.com/joelstransky/Tailorbird


--------------------------------------------------------------------------------
HOW TO OPEN TAILORBIRD ON macOS (Choose either method)
--------------------------------------------------------------------------------

METHOD 1: System Settings (Recommended & Easiest)
--------------------------------------------------
1. Drag "Tailorbird.app" into your "Applications" folder.
2. Double-click Tailorbird in Applications. If the warning dialog appears, click "Cancel" (or "Done").
3. Open your Mac's "System Settings" (or "System Preferences").
4. Go to "Privacy & Security" and scroll down to the "Security" section.
5. You will see a note saying:
   '"Tailorbird" was blocked from use because it is not from an identified developer.'
6. Click the button labeled "Open Anyway" (enter your Mac password / Touch ID if prompted).
7. In the confirmation dialog, click "Open".
8. Tailorbird will launch and you will never see this prompt again!


METHOD 2: Right-Click Shortcut
--------------------------------------------------
1. Drag "Tailorbird.app" into your "Applications" folder.
2. In Finder, open the "Applications" folder.
3. Hold the Control key (Ctrl) and right-click on "Tailorbird.app", then select "Open" from the context menu.
4. A dialog will appear with an "Open" button. Click "Open".


METHOD 3: Terminal Command (One-liner for developers)
--------------------------------------------------
If macOS displays a warning that the app cannot be verified or is quarantined, you can remove the Gatekeeper quarantine flag in Terminal:

    xattr -cr /Applications/Tailorbird.app


--------------------------------------------------------------------------------
HOW TO UNINSTALL
--------------------------------------------------------------------------------
To completely remove Tailorbird along with all locally stored candidate data and settings, double-click the "Uninstall.command" script included in this disk image, or run it in Terminal.

Alternatively, you can manually delete:
  - /Applications/Tailorbird.app
  - ~/Library/Application Support/Tailorbird

================================================================================
