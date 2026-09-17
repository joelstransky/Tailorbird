use serde::{Deserialize, Serialize};

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

    // 12. Highlight remaining unfilled form fields
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
