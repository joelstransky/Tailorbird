use serde::{Deserialize, Serialize};
use crate::prospect::SpecialField;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CandidateProfile {
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub pronouns: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub current_company: String,
    #[serde(default)]
    pub linkedin: String,
    #[serde(default)]
    pub github: String,
    #[serde(default)]
    pub portfolio_url: String,
    #[serde(default)]
    pub experience_years: String,
    #[serde(default)]
    pub salary_min: String,
    #[serde(default)]
    pub salary_max: String,
    #[serde(default)]
    pub gender: String,
    #[serde(default)]
    pub race: String,
    #[serde(default)]
    pub veteran_status: String,
    #[serde(default)]
    pub disability_status: String,
}

/// Generates the self-contained JavaScript snippet to be evaluated in the target webview.
pub fn generate_autofill_script(profile: &CandidateProfile) -> String {
    let profile_json = serde_json::to_string(profile).unwrap_or_else(|_| "{}".to_string());

    format!(
        r#"(function() {{
    const profile = {profile_json};
    console.log('[Tailorbird] Executing smart autofill injection...', profile);

    // 1. Inject or update styling for autofill & unfilled field highlights
    let styleEl = document.getElementById('tailorbird-autofill-styles');
    if (!styleEl) {{
        styleEl = document.createElement('style');
        styleEl.id = 'tailorbird-autofill-styles';
        styleEl.textContent = `
            .tailorbird-filled {{
                transition: border-color 0.3s ease, box-shadow 0.3s ease !important;
                border-color: #10b981 !important;
                box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.35) !important;
            }}
            .tailorbird-unfilled {{
                outline: 2px dashed #f59e0b !important;
                outline-offset: 2px !important;
                box-shadow: 0 0 0 3px rgba(245, 158, 11, 0.22) !important;
                background-color: rgba(245, 158, 11, 0.05) !important;
                transition: outline 0.2s ease, box-shadow 0.2s ease, background-color 0.2s ease !important;
            }}
            .tailorbird-unfilled:focus {{
                outline: 2px solid #3b82f6 !important;
                box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.3) !important;
            }}
            #tailorbird-toast-notice {{
                position: fixed;
                bottom: 24px;
                right: 24px;
                background: #1e1e1e;
                color: #f3f4f6;
                border: 1px solid #f59e0b;
                box-shadow: 0 10px 30px rgba(0,0,0,0.65);
                padding: 10px 16px;
                border-radius: 8px;
                font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                font-size: 12.5px;
                z-index: 2147483647;
                display: flex;
                align-items: center;
                gap: 12px;
                animation: tbToastIn 0.3s ease-out;
            }}
            @keyframes tbToastIn {{
                from {{ transform: translateY(16px); opacity: 0; }}
                to {{ transform: translateY(0); opacity: 1; }}
            }}
        `;
        document.head.appendChild(styleEl);
    }}

    // Clean up any previous unfilled highlights or toasts
    document.querySelectorAll('.tailorbird-unfilled').forEach(el => el.classList.remove('tailorbird-unfilled'));
    const oldToast = document.getElementById('tailorbird-toast-notice');
    if (oldToast) oldToast.remove();

    // Framework-safe value setter
    function setNativeValue(element, value) {{
        if (!element) return false;
        try {{
            let prototype = Object.getPrototypeOf(element);
            let descriptor = Object.getOwnPropertyDescriptor(prototype, 'value');
            
            // Traverse prototype chain if needed (e.g. HTMLInputElement -> HTMLElement)
            while (prototype && !descriptor) {{
                prototype = Object.getPrototypeOf(prototype);
                if (prototype) {{
                    descriptor = Object.getOwnPropertyDescriptor(prototype, 'value');
                }}
            }}

            if (descriptor && descriptor.set) {{
                descriptor.set.call(element, value);
            }} else {{
                element.value = value;
            }}

            // Dispatch focus, input, change, and blur bubbling events
            element.dispatchEvent(new Event('focus', {{ bubbles: true, cancelable: true }}));
            element.dispatchEvent(new Event('input', {{ bubbles: true, cancelable: true }}));
            element.dispatchEvent(new Event('change', {{ bubbles: true, cancelable: true }}));
            element.dispatchEvent(new Event('blur', {{ bubbles: true, cancelable: true }}));

            // Gentle visual highlight to indicate autofill success
            element.classList.add('tailorbird-filled');
            setTimeout(() => {{
                element.classList.remove('tailorbird-filled');
            }}, 2000);

            return true;
        }} catch (err) {{
            console.error('[Tailorbird] Failed to set native value for element:', element, err);
            return false;
        }}
    }}

    // Helper to test if element is visible and editable
    function isEditable(el) {{
        if (!el || el.disabled || el.readOnly) return false;
        return el.type !== 'hidden';
    }}

    // Split name into first and last
    const nameParts = (profile.fullName || '').trim().split(/\s+/);
    const firstName = nameParts[0] || '';
    const lastName = nameParts.slice(1).join(' ') || '';

    let filledCount = 0;
    const filledElements = new Set();

    function tryFill(selector, value) {{
        if (!value) return;
        const elements = document.querySelectorAll(selector);
        for (const el of elements) {{
            if (isEditable(el) && !filledElements.has(el)) {{
                if (setNativeValue(el, value)) {{
                    filledElements.add(el);
                    filledCount++;
                    console.log(`[Tailorbird] Filled [${{selector}}] with: ${{value}}`);
                }}
            }}
        }}
    }}

    // 1. Full Name / First Name / Last Name
    const hasSeparateNameInputs = 
        document.querySelector('input[name*="first" i], input[id*="first" i], input[autocomplete="given-name"]') &&
        document.querySelector('input[name*="last" i], input[id*="last" i], input[autocomplete="family-name"]');

    if (hasSeparateNameInputs && firstName && lastName) {{
        tryFill('input[autocomplete="given-name"], input[name*="first" i], input[id*="first" i], input[placeholder*="first" i]', firstName);
        tryFill('input[autocomplete="family-name"], input[name*="last" i], input[id*="last" i], input[placeholder*="last" i]', lastName);
    }} else if (profile.fullName) {{
        tryFill('input[autocomplete="name"], input[name="fullName"], input[name="name"], input[name*="name" i]:not([name*="user" i]):not([name*="company" i]):not([name*="file" i]):not([name*="first" i]):not([name*="last" i]), input[id*="name" i]:not([id*="user" i]):not([id*="company" i]):not([id*="first" i]):not([id*="last" i]), input[placeholder*="full name" i]', profile.fullName);
    }}

    // 2. Email
    if (profile.email) {{
        tryFill('input[type="email"], input[autocomplete="email"], input[name*="email" i], input[id*="email" i], input[placeholder*="email" i]', profile.email);
    }}

    // 3. Phone
    if (profile.phone) {{
        tryFill('input[type="tel"], input[autocomplete="tel"], input[name*="phone" i], input[id*="phone" i], input[name*="mobile" i], input[placeholder*="phone" i]', profile.phone);
    }}

    // 4. Location / City
    if (profile.location) {{
        tryFill('input[name="location"], input[id="location"], input[name*="location" i], input[id*="location" i], input[placeholder*="location" i], input[placeholder*="city" i], input[name*="city" i]', profile.location);
    }}

    // 5. Current Company / Organization
    if (profile.currentCompany) {{
        tryFill('input[name="org"], input[id="org"], input[name*="company" i], input[id*="company" i], input[placeholder*="company" i], input[name*="employer" i], input[id*="employer" i], input[name*="organization" i], input[id*="organization" i]', profile.currentCompany);
    }}

    // 6. Pronouns
    if (profile.pronouns) {{
        const targetPronoun = profile.pronouns.trim().toLowerCase();
        let pronounMatched = false;

        // A. Try checkboxes / radio buttons matching pronouns (e.g. Lever checkbox list)
        const pronounInputs = document.querySelectorAll('input[name*="pronoun" i], input[id*="pronoun" i]');
        for (const input of pronounInputs) {{
            if (input.type === 'checkbox' || input.type === 'radio') {{
                const inputVal = (input.value || '').toLowerCase();
                const parentText = (input.closest('label')?.textContent || input.parentElement?.textContent || '').toLowerCase();
                if (inputVal.includes(targetPronoun) || parentText.includes(targetPronoun)) {{
                    input.checked = true;
                    input.dispatchEvent(new Event('input', {{ bubbles: true }}));
                    input.dispatchEvent(new Event('change', {{ bubbles: true }}));
                    filledElements.add(input);
                    filledCount++;
                    pronounMatched = true;
                    console.log(`[Tailorbird] Checked pronoun option: ${{inputVal || parentText}}`);
                    break;
                }}
            }}
        }}

        // B. Try <select> dropdown for pronouns
        if (!pronounMatched) {{
            const pronounSelects = document.querySelectorAll('select[name*="pronoun" i], select[id*="pronoun" i]');
            for (const sel of pronounSelects) {{
                for (const opt of sel.options) {{
                    const optText = (opt.text || '').toLowerCase();
                    const optVal = (opt.value || '').toLowerCase();
                    if (optText.includes(targetPronoun) || optVal.includes(targetPronoun)) {{
                        sel.value = opt.value;
                        sel.dispatchEvent(new Event('change', {{ bubbles: true }}));
                        filledElements.add(sel);
                        filledCount++;
                        pronounMatched = true;
                        break;
                    }}
                }}
                if (pronounMatched) break;
            }}
        }}

        // C. Try text inputs for pronouns (or write-in inputs)
        if (!pronounMatched) {{
            tryFill('input[type="text"][name*="pronoun" i], input[type="text"][id*="pronoun" i], input[type="text"][placeholder*="pronoun" i]', profile.pronouns);
        }}
    }}

    // 7. LinkedIn
    if (profile.linkedin) {{
        tryFill('input[name*="linkedin" i], input[id*="linkedin" i], input[placeholder*="linkedin" i], input[name*="urls[LinkedIn]" i]', profile.linkedin);
    }}

    // 8. GitHub
    if (profile.github) {{
        tryFill('input[name*="github" i], input[id*="github" i], input[placeholder*="github" i], input[name*="urls[GitHub]" i]', profile.github);
    }}

    // 9. Portfolio / Website
    if (profile.portfolioUrl) {{
        tryFill('input[name*="urls[Portfolio]" i], input[name*="portfolio" i], input[id*="portfolio" i], input[placeholder*="portfolio" i], input[name*="website" i], input[id*="website" i], input[placeholder*="website" i], input[name*="urls[Other]" i]', profile.portfolioUrl);
    }}

    // 10. Years of Experience
    if (profile.experienceYears) {{
        tryFill('input[name*="experience" i], input[id*="experience" i], input[name*="years" i], input[id*="years" i], input[placeholder*="years" i]', profile.experienceYears);

        // Also check <select> for experience
        const selects = document.querySelectorAll('select[name*="experience" i], select[id*="experience" i]');
        for (const sel of selects) {{
            if (!filledElements.has(sel)) {{
                for (const opt of sel.options) {{
                    if (opt.value === profile.experienceYears || opt.text.includes(profile.experienceYears)) {{
                        sel.value = opt.value;
                        sel.dispatchEvent(new Event('change', {{ bubbles: true }}));
                        filledElements.add(sel);
                        filledCount++;
                        break;
                    }}
                }}
            }}
        }}
    }}

    // 11. Desired Salary / Compensation
    if (profile.salaryMin || profile.salaryMax) {{
        let salText = '';
        if (profile.salaryMin && profile.salaryMax) {{
            salText = `${{profile.salaryMin}} - ${{profile.salaryMax}}`;
        }} else {{
            salText = profile.salaryMin || profile.salaryMax;
        }}
        tryFill('input[name*="salary" i], input[id*="salary" i], input[placeholder*="salary" i], input[name*="compensation" i], input[id*="compensation" i], input[name*="pay" i]', salText);
    }}

    // Helper for dropdown/radio/text selection fields (e.g. Demographics, EEO)
    const getFieldAndContextText = (el) => {{
        let text = ((el.name || '') + ' ' + (el.id || '') + ' ' + (el.getAttribute('aria-label') || '') + ' ' + (el.placeholder || '')).toLowerCase();
        let curr = el.parentElement;
        for (let i = 0; i < 6 && curr && curr !== document.body; i++) {{
            const classStr = (curr.className || '').toString().toLowerCase();
            const tag = (curr.tagName || '').toLowerCase();
            if (
                classStr.includes('question') ||
                classStr.includes('label') ||
                classStr.includes('field') ||
                classStr.includes('section') ||
                classStr.includes('group') ||
                tag === 'fieldset' ||
                tag === 'li'
            ) {{
                // Collect text content of the enclosing question/label container
                const containerText = (curr.textContent || '').replace(/\s+/g, ' ').trim().toLowerCase();
                text += ' ' + containerText;
                break;
            }}
            curr = curr.parentElement;
        }}
        return text;
    }};

    const isOptionMatch = (targetVal, optVal, optText) => {{
        if (!targetVal) return false;
        const t = targetVal.toLowerCase().trim();
        const v = (optVal || '').toLowerCase().trim();
        const txt = (optText || '').toLowerCase().trim();

        // Exact matches
        if (v === t || txt === t) return true;

        // 1. Gender mappings (strictly prevent "male" from matching "female"!)
        if (t === 'male') {{
            return (txt === 'male' || v === 'male' || txt === 'man' || v === 'man' || txt.startsWith('male ') || txt.startsWith('male/'));
        }}
        if (t === 'female') {{
            return (txt === 'female' || v === 'female' || txt === 'woman' || v === 'woman' || txt.startsWith('female ') || txt.startsWith('female/'));
        }}
        if (t === 'non-binary') {{
            return (txt.includes('non-binary') || v.includes('non-binary'));
        }}

        // 2. Race / Ethnicity mappings
        if (t.includes('white')) {{
            if (txt.includes('white') || v.includes('white') || txt.includes('caucasian')) return true;
        }}
        if (t.includes('hispanic') || t.includes('latino')) {{
            if (txt.includes('hispanic') || txt.includes('latino') || txt.includes('spanish')) return true;
        }}
        if (t.includes('black') || t.includes('african')) {{
            if (txt.includes('black') || txt.includes('african')) return true;
        }}
        if (t.includes('asian')) {{
            if (txt.includes('asian')) return true;
        }}

        // 3. Veteran mappings
        if (t.includes('not a protected veteran') || t.includes('not a veteran')) {{
            if (txt.includes('not a protected veteran') || txt.includes('not a veteran') || v.includes('not a protected veteran')) return true;
        }}
        if (t.includes('one or more') || (t.includes('protected veteran') && !t.includes('not'))) {{
            if ((txt.includes('one or more') || txt.includes('protected veteran')) && !txt.includes('not a protected veteran') && !txt.includes('not a veteran')) return true;
        }}

        // 4. Disability mappings
        const isTargetDecline = t.includes('wish to answer') || t.includes('prefer not') || t.includes('decline') || t.includes('disclose');
        if (isTargetDecline) {{
            if (txt.includes('prefer not') || txt.includes('wish to answer') || txt.includes('decline') || txt.includes('disclose') || v.includes('prefer not') || v.includes('disclose')) return true;
        }}
        if (!isTargetDecline && (t === 'yes' || t.includes('have a disability'))) {{
            if (txt === 'yes' || v === 'yes' || (txt.includes('yes') && !txt.includes('no'))) return true;
        }}
        if (!isTargetDecline && (t === 'no' || t.includes('do not have a disability'))) {{
            if (txt === 'no' || v === 'no' || (txt.includes('no') && !txt.includes('yes'))) return true;
        }}

        // 5. Pronoun mappings
        if (t.includes('he/him')) {{
            if (txt.includes('he/him') || v.includes('he/him') || txt.includes('he / him')) return true;
        }}
        if (t.includes('she/her')) {{
            if (txt.includes('she/her') || v.includes('she/her') || txt.includes('she / her')) return true;
        }}
        if (t.includes('they/them')) {{
            if (txt.includes('they/them') || v.includes('they/them') || txt.includes('they / them')) return true;
        }}

        // General fallback
        if (v && v.includes(t)) return true;
        if (txt && txt.includes(t)) return true;

        return false;
    }};

    const fillChoiceField = (fieldPattern, targetVal) => {{
        if (!targetVal) return;
        let matched = false;

        // A. Radio / Checkbox
        const inputs = document.querySelectorAll('input[type="radio"], input[type="checkbox"]');
        for (const input of inputs) {{
            const contextText = getFieldAndContextText(input);
            if (fieldPattern.test(contextText)) {{
                const inputVal = input.value || '';
                const parentText = input.closest('label')?.textContent || input.parentElement?.textContent || '';
                if (isOptionMatch(targetVal, inputVal, parentText)) {{
                    input.checked = true;
                    input.dispatchEvent(new Event('input', {{ bubbles: true }}));
                    input.dispatchEvent(new Event('change', {{ bubbles: true }}));
                    filledElements.add(input);
                    filledCount++;
                    matched = true;
                    console.log(`[Tailorbird] Checked option for pattern ${{fieldPattern}}: ${{inputVal || parentText}}`);
                    break;
                }}
            }}
        }}

        // B. Select dropdown
        if (!matched) {{
            const selects = document.querySelectorAll('select');
            for (const sel of selects) {{
                const contextText = getFieldAndContextText(sel);
                if (fieldPattern.test(contextText) && !filledElements.has(sel)) {{
                    for (const opt of sel.options) {{
                        const optText = opt.text || '';
                        const optVal = opt.value || '';
                        if (isOptionMatch(targetVal, optVal, optText)) {{
                            sel.value = opt.value;
                            sel.dispatchEvent(new Event('change', {{ bubbles: true }}));
                            filledElements.add(sel);
                            filledCount++;
                            matched = true;
                            console.log(`[Tailorbird] Selected dropdown option for pattern ${{fieldPattern}}: ${{optText || optVal}}`);
                            break;
                        }}
                    }}
                    if (matched) break;
                }}
            }}
        }}

        // C. Text input
        if (!matched) {{
            const textInputs = document.querySelectorAll('input[type="text"], input:not([type])');
            for (const input of textInputs) {{
                const contextText = getFieldAndContextText(input);
                if (fieldPattern.test(contextText) && !filledElements.has(input)) {{
                    input.value = targetVal;
                    input.dispatchEvent(new Event('input', {{ bubbles: true }}));
                    input.dispatchEvent(new Event('change', {{ bubbles: true }}));
                    input.classList.add('tailorbird-filled');
                    filledElements.add(input);
                    filledCount++;
                    break;
                }}
            }}
        }}
    }};

    // 12. Demographics / EEO Self-Identification
    if (profile.gender) {{
        fillChoiceField(/gender|sex\b/i, profile.gender);
    }}
    if (profile.race) {{
        fillChoiceField(/race|ethnic/i, profile.race);
    }}
    if (profile.veteranStatus) {{
        fillChoiceField(/veteran|military/i, profile.veteranStatus);
    }}
    if (profile.disabilityStatus) {{
        fillChoiceField(/disabilit/i, profile.disabilityStatus);
    }}
    if (profile.pronouns) {{
        fillChoiceField(/pronoun/i, profile.pronouns);
    }}

    // 13. Highlight remaining unfilled form fields
    const allFormControls = Array.from(document.querySelectorAll('input, select, textarea'));
    let unfilledCount = 0;

    // Track which radio/checkbox groups already have at least one selection
    const checkedGroups = new Set();
    allFormControls.forEach(el => {{
        if ((el.type === 'radio' || el.type === 'checkbox') && el.name && el.checked) {{
            checkedGroups.add(el.name);
        }}
    }});

    allFormControls.forEach(el => {{
        // Skip hidden, button, submit, reset, search controls
        if (el.type === 'hidden' || el.type === 'submit' || el.type === 'button' || el.type === 'reset') return;
        if (el.type === 'search' || el.getAttribute('role') === 'search') return;
        // Skip non-visible elements (unless file input which often has 0 dimensions)
        if (el.type !== 'file' && el.offsetParent === null && el.getClientRects().length === 0) return;

        let isUnfilled = false;
        if (el.type === 'radio' || el.type === 'checkbox') {{
            if (el.name) {{
                if (!checkedGroups.has(el.name)) {{
                    isUnfilled = true;
                }}
            }} else if (!el.checked) {{
                isUnfilled = true;
            }}
        }} else if (el.tagName === 'SELECT') {{
            if (el.selectedIndex <= 0 || !el.value || el.value.trim() === '') {{
                isUnfilled = true;
            }}
        }} else {{
            // Text, email, tel, number, url, textarea, file
            if (!el.value || el.value.trim() === '') {{
                isUnfilled = true;
            }}
        }}

        if (isUnfilled && !filledElements.has(el)) {{
            el.classList.add('tailorbird-unfilled');
            unfilledCount++;

            // Dynamic listener: clear highlight as soon as the user enters or selects a value
            if (!el._tailorbirdListenerAttached) {{
                el._tailorbirdListenerAttached = true;
                const clearHighlight = () => {{
                    let nowFilled = false;
                    if (el.type === 'radio' || el.type === 'checkbox') {{
                        if (el.name) {{
                            const anyChecked = document.querySelector(`input[name="${{el.name}}"]:checked`);
                            if (anyChecked) {{
                                document.querySelectorAll(`input[name="${{el.name}}"]`).forEach(inp => inp.classList.remove('tailorbird-unfilled'));
                                updateToast();
                                return;
                            }}
                        }}
                        nowFilled = el.checked;
                    }} else if (el.tagName === 'SELECT') {{
                        nowFilled = el.selectedIndex > 0 && el.value !== '';
                    }} else {{
                        nowFilled = el.value && el.value.trim().length > 0;
                    }}
                    if (nowFilled) {{
                        el.classList.remove('tailorbird-unfilled');
                        updateToast();
                    }}
                }};

                el.addEventListener('input', clearHighlight);
                el.addEventListener('change', clearHighlight);
                el.addEventListener('blur', clearHighlight);
            }}
        }}
    }});

    // Live updater to keep toast in sync and remove it only when 0 unfilled fields remain
    function updateToast() {{
        const toast = document.getElementById('tailorbird-toast-notice');
        if (!toast) return;

        const remainingElements = Array.from(document.querySelectorAll('.tailorbird-unfilled'));
        const countedGroups = new Set();
        let remainingCount = 0;
        for (const elem of remainingElements) {{
            if (elem.type === 'radio' || elem.type === 'checkbox') {{
                if (elem.name) {{
                    if (!countedGroups.has(elem.name)) {{
                        countedGroups.add(elem.name);
                        remainingCount++;
                    }}
                }} else {{
                    remainingCount++;
                }}
            }} else {{
                remainingCount++;
            }}
        }}

        if (remainingCount > 0) {{
            const countSpan = toast.querySelector('#tailorbird-unfilled-count');
            if (countSpan) {{
                countSpan.innerHTML = `⚠️ <strong>${{remainingCount}}</strong> remaining marked in amber`;
            }}
        }} else {{
            toast.style.borderColor = '#10b981';
            toast.innerHTML = `
                <span style="color: #10b981;">✅ <strong>All fields complete!</strong></span>
                <button style="background:none; border:none; color:#9ca3af; font-size:13px; cursor:pointer; margin-left:6px; padding:0 4px;" onclick="this.parentElement.remove()">✕</button>
            `;
            setTimeout(() => {{
                if (toast.parentElement) {{
                    toast.style.transition = 'opacity 0.4s ease';
                    toast.style.opacity = '0';
                    setTimeout(() => toast.remove(), 400);
                }}
            }}, 2500);
        }}
    }}

    // 12. Display unobtrusive toast notification (stays visible while unfilled fields exist)
    const toast = document.createElement('div');
    toast.id = 'tailorbird-toast-notice';
    if (unfilledCount > 0) {{
        toast.innerHTML = `
            <span>⚡ <strong>${{filledCount}}</strong> field${{filledCount === 1 ? '' : 's'}} autofilled</span>
            <span style="color: #64748b;">•</span>
            <span id="tailorbird-unfilled-count" style="color: #f59e0b;">⚠️ <strong>${{unfilledCount}}</strong> remaining marked in amber</span>
            <button style="background:none; border:none; color:#9ca3af; font-size:13px; cursor:pointer; margin-left:6px; padding:0 4px;" onclick="this.parentElement.remove()">✕</button>
        `;
    }} else {{
        toast.style.borderColor = '#10b981';
        toast.innerHTML = `
            <span>⚡ <strong>${{filledCount}}</strong> field${{filledCount === 1 ? '' : 's'}} autofilled</span>
            <span style="color: #64748b;">•</span>
            <span style="color: #10b981;">✅ <strong>All fields complete!</strong></span>
            <button style="background:none; border:none; color:#9ca3af; font-size:13px; cursor:pointer; margin-left:6px; padding:0 4px;" onclick="this.parentElement.remove()">✕</button>
        `;
        setTimeout(() => {{
            if (toast.parentElement) {{
                toast.style.transition = 'opacity 0.4s ease';
                toast.style.opacity = '0';
                setTimeout(() => toast.remove(), 400);
            }}
        }}, 4000);
    }}
    document.body.appendChild(toast);

    console.log(`[Tailorbird] Completed autofill: ${{filledCount}} populated, ${{unfilledCount}} unfilled highlighted.`);
    return filledCount;
}})();"#
    )
}

/// Generates the self-contained contextual menu script that runs in the target webview.
/// Allows right-clicking any input/textarea to insert saved profile fields or special fields.
pub fn generate_context_menu_script(
    profile: Option<&CandidateProfile>,
    special_fields: &[SpecialField],
) -> String {
    let profile_json = serde_json::to_string(&profile).unwrap_or_else(|_| "null".to_string());
    let special_fields_json =
        serde_json::to_string(special_fields).unwrap_or_else(|_| "[]".to_string());

    format!(
        r#"(function() {{
    window.__TAILORBIRD_CONTEXT_DATA__ = window.__TAILORBIRD_CONTEXT_DATA__ || {{}};
    window.__TAILORBIRD_CONTEXT_DATA__.candidateProfile = {profile_json};
    window.__TAILORBIRD_CONTEXT_DATA__.specialFields = {special_fields_json};

    window.__UPDATE_TAILORBIRD_CONTEXT_MENU__ = function(data) {{
        if (!data) return;
        if (data.candidateProfile !== undefined) {{
            window.__TAILORBIRD_CONTEXT_DATA__.candidateProfile = data.candidateProfile;
        }}
        if (data.specialFields !== undefined) {{
            window.__TAILORBIRD_CONTEXT_DATA__.specialFields = data.specialFields;
        }}
    }};

    if (window.__TAILORBIRD_CM_INITIALIZED__) return;
    window.__TAILORBIRD_CM_INITIALIZED__ = true;

    function initContextMenu() {{
        const styleId = 'tailorbird-context-menu-styles';
        if (!document.getElementById(styleId)) {{
            const style = document.createElement('style');
            style.id = styleId;
            style.textContent = `
                #tailorbird-context-menu {{
                    position: fixed !important;
                    z-index: 2147483647 !important;
                    background: #1c2128 !important;
                    border: 1px solid #444c56 !important;
                    border-radius: 6px !important;
                    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.75), 0 2px 8px rgba(0, 0, 0, 0.4) !important;
                    color: #e6edf3 !important;
                    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif !important;
                    font-size: 11.5px !important;
                    min-width: 220px !important;
                    max-width: 320px !important;
                    max-height: 420px !important;
                    overflow-y: auto !important;
                    overflow-x: hidden !important;
                    padding: 4px 0 !important;
                    display: none;
                    user-select: none !important;
                    -webkit-user-select: none !important;
                }}
                #tailorbird-context-menu::-webkit-scrollbar {{
                    width: 5px;
                }}
                #tailorbird-context-menu::-webkit-scrollbar-thumb {{
                    background: #444c56;
                    border-radius: 3px;
                }}
                .tb-cm-header {{
                    font-size: 9.5px !important;
                    font-weight: 700 !important;
                    color: #768390 !important;
                    text-transform: uppercase !important;
                    letter-spacing: 0.06em !important;
                    padding: 6px 12px 3px !important;
                    display: flex !important;
                    align-items: center !important;
                    gap: 5px !important;
                }}
                .tb-cm-divider {{
                    height: 1px !important;
                    background: #2d333b !important;
                    margin: 4px 0 !important;
                }}
                .tb-cm-item {{
                    display: flex !important;
                    align-items: center !important;
                    justify-content: space-between !important;
                    padding: 5px 12px !important;
                    cursor: pointer !important;
                    gap: 8px !important;
                    transition: background 0.1s ease, color 0.1s ease !important;
                }}
                .tb-cm-item:hover:not(.tb-cm-disabled) {{
                    background: #1f6feb !important;
                    color: #ffffff !important;
                }}
                .tb-cm-item:hover:not(.tb-cm-disabled) .tb-cm-val {{
                    color: rgba(255, 255, 255, 0.85) !important;
                }}
                .tb-cm-item.tb-cm-disabled {{
                    opacity: 0.38 !important;
                    cursor: not-allowed !important;
                }}
                .tb-cm-label {{
                    font-weight: 500 !important;
                    white-space: nowrap !important;
                    flex-shrink: 0 !important;
                }}
                .tb-cm-val {{
                    font-size: 10.5px !important;
                    color: #768390 !important;
                    overflow: hidden !important;
                    text-overflow: ellipsis !important;
                    white-space: nowrap !important;
                    text-align: right !important;
                    max-width: 135px !important;
                }}
            `;
            (document.head || document.documentElement).appendChild(style);
        }}

        let menu = document.getElementById('tailorbird-context-menu');
        if (!menu) {{
            menu = document.createElement('div');
            menu.id = 'tailorbird-context-menu';
            (document.body || document.documentElement).appendChild(menu);
        }}

        let activeTargetElement = null;

        function isEditableInput(el) {{
            if (!el) return false;
            const tag = el.tagName ? el.tagName.toLowerCase() : '';
            if (tag === 'textarea') return true;
            if (tag === 'input') {{
                const type = (el.type || 'text').toLowerCase();
                const nonTextTypes = ['checkbox', 'radio', 'submit', 'button', 'reset', 'file', 'image', 'hidden', 'range', 'color'];
                return !nonTextTypes.includes(type);
            }}
            if (el.isContentEditable) return true;
            return false;
        }}

        function closeContextMenu() {{
            if (menu) {{
                menu.style.display = 'none';
                menu.innerHTML = '';
            }}
            activeTargetElement = null;
        }}

        function insertValue(val) {{
            if (!activeTargetElement || val === undefined || val === null) {{
                closeContextMenu();
                return;
            }}

            const el = activeTargetElement;
            el.focus();

            if (el.isContentEditable) {{
                el.innerText = val;
                el.dispatchEvent(new Event('input', {{ bubbles: true }}));
                el.dispatchEvent(new Event('change', {{ bubbles: true }}));
            }} else {{
                const proto = el.tagName.toLowerCase() === 'textarea' 
                    ? window.HTMLTextAreaElement.prototype 
                    : window.HTMLInputElement.prototype;
                const nativeSetter = Object.getOwnPropertyDescriptor(proto, 'value')?.set;
                if (nativeSetter) {{
                    nativeSetter.call(el, val);
                }} else {{
                    el.value = val;
                }}
                el.selectionStart = el.selectionEnd = el.value.length;
                el.dispatchEvent(new Event('input', {{ bubbles: true }}));
                el.dispatchEvent(new Event('change', {{ bubbles: true }}));
            }}

            // Visual feedback pulse
            const prevTransition = el.style.transition;
            const prevOutline = el.style.outline;
            const prevBoxShadow = el.style.boxShadow;
            el.style.transition = 'outline 0.2s ease, box-shadow 0.2s ease';
            el.style.outline = '2px solid #10b981';
            el.style.boxShadow = '0 0 0 3px rgba(16, 185, 129, 0.3)';

            if (el.classList.contains('tailorbird-unfilled')) {{
                el.classList.remove('tailorbird-unfilled');
                if (window.__TAILORBIRD_UPDATE_TOAST__) {{
                    window.__TAILORBIRD_UPDATE_TOAST__();
                }}
            }}

            setTimeout(() => {{
                el.style.transition = prevTransition;
                el.style.outline = prevOutline;
                el.style.boxShadow = prevBoxShadow;
            }}, 1200);

            closeContextMenu();
        }}

        document.addEventListener('contextmenu', function(e) {{
            let target = e.target;
            while (target && target !== document.body && target !== document.documentElement && !isEditableInput(target)) {{
                target = target.parentElement;
            }}

            if (!isEditableInput(target)) {{
                closeContextMenu();
                return;
            }}

            e.preventDefault();
            activeTargetElement = target;

            const data = window.__TAILORBIRD_CONTEXT_DATA__ || {{}};
            const profile = data.candidateProfile || {{}};
            const specialFields = Array.isArray(data.specialFields) ? data.specialFields : [];

            const profileItems = [
                {{ label: 'Full Name', value: profile.fullName }},
                {{ label: 'Pronouns', value: profile.pronouns }},
                {{ label: 'Email', value: profile.email }},
                {{ label: 'Phone', value: profile.phone }},
                {{ label: 'Location', value: profile.location }},
                {{ label: 'Current Company', value: profile.currentCompany }},
                {{ label: 'LinkedIn URL', value: profile.linkedin }},
                {{ label: 'GitHub URL', value: profile.github }},
                {{ label: 'Portfolio URL', value: profile.portfolioUrl }},
                {{ label: 'Years Experience', value: profile.experienceYears }},
                {{ label: 'Salary Min', value: profile.salaryMin }},
                {{ label: 'Salary Max', value: profile.salaryMax }},
                {{ label: 'Gender', value: profile.gender }},
                {{ label: 'Race / Ethnicity', value: profile.race }},
                {{ label: 'Veteran Status', value: profile.veteranStatus }},
                {{ label: 'Disability Status', value: profile.disabilityStatus }},
            ];

            if (profile.salaryMin && profile.salaryMax) {{
                profileItems.push({{ label: 'Salary Range', value: `${{profile.salaryMin}} - ${{profile.salaryMax}}` }});
            }}

            menu.innerHTML = '';

            const profileHeader = document.createElement('div');
            profileHeader.className = 'tb-cm-header';
            profileHeader.innerHTML = '<span>👤</span><span>Profile Fields</span>';
            menu.appendChild(profileHeader);

            profileItems.forEach(item => {{
                const hasVal = item.value !== undefined && item.value !== null && item.value.toString().trim().length > 0;
                const div = document.createElement('div');
                div.className = 'tb-cm-item' + (hasVal ? '' : ' tb-cm-disabled');
                div.innerHTML = `
                    <span class="tb-cm-label">${{item.label}}</span>
                    <span class="tb-cm-val" title="${{hasVal ? item.value : ''}}">${{hasVal ? item.value : '—'}}</span>
                `;
                if (hasVal) {{
                    div.addEventListener('click', (ev) => {{
                        ev.stopPropagation();
                        insertValue(item.value);
                    }});
                }}
                menu.appendChild(div);
            }});

            if (specialFields.length > 0) {{
                const divider = document.createElement('div');
                divider.className = 'tb-cm-divider';
                menu.appendChild(divider);

                const sfHeader = document.createElement('div');
                sfHeader.className = 'tb-cm-header';
                sfHeader.innerHTML = '<span>📝</span><span>Special Fields</span>';
                menu.appendChild(sfHeader);

                specialFields.forEach((sf, idx) => {{
                    const labelText = (sf.label || '').trim() || `Snippet #${{idx + 1}}`;
                    const contentText = (sf.content || '').trim();
                    const hasVal = contentText.length > 0;

                    const div = document.createElement('div');
                    div.className = 'tb-cm-item' + (hasVal ? '' : ' tb-cm-disabled');
                    div.innerHTML = `
                        <span class="tb-cm-label">${{labelText}}</span>
                        <span class="tb-cm-val" title="${{hasVal ? contentText : ''}}">${{hasVal ? contentText : '—'}}</span>
                    `;
                    if (hasVal) {{
                        div.addEventListener('click', (ev) => {{
                            ev.stopPropagation();
                            insertValue(sf.content);
                        }});
                    }}
                    menu.appendChild(div);
                }});
            }}

            menu.style.display = 'block';
            menu.style.visibility = 'hidden';

            requestAnimationFrame(() => {{
                const menuWidth = menu.offsetWidth || 240;
                const menuHeight = menu.offsetHeight || 300;
                let posX = e.clientX;
                let posY = e.clientY;

                if (posX + menuWidth > window.innerWidth) {{
                    posX = Math.max(10, window.innerWidth - menuWidth - 10);
                }}
                if (posY + menuHeight > window.innerHeight) {{
                    posY = Math.max(10, window.innerHeight - menuHeight - 10);
                }}

                menu.style.left = posX + 'px';
                menu.style.top = posY + 'px';
                menu.style.visibility = 'visible';
            }});
        }}, true);

        document.addEventListener('click', function(e) {{
            if (menu && menu.style.display === 'block' && !menu.contains(e.target)) {{
                closeContextMenu();
            }}
        }}, true);

        document.addEventListener('keydown', function(e) {{
            if (e.key === 'Escape') {{
                closeContextMenu();
            }}
        }});

        window.addEventListener('blur', closeContextMenu);
        window.addEventListener('scroll', function(e) {{
            if (menu && menu.style.display !== 'none' && (e.target === menu || menu.contains(e.target))) {{
                return;
            }}
            closeContextMenu();
        }}, true);

        menu.addEventListener('wheel', function(e) {{
            e.stopPropagation();
        }}, {{ passive: true }});
        menu.addEventListener('touchmove', function(e) {{
            e.stopPropagation();
        }}, {{ passive: true }});
    }}

    if (document.readyState === 'loading') {{
        document.addEventListener('DOMContentLoaded', initContextMenu);
    }} else {{
        initContextMenu();
    }}
}})();"#
    )
}
