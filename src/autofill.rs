use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CandidateProfile {
    #[serde(default)]
    pub full_name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub linkedin: String,
    #[serde(default)]
    pub github: String,
    #[serde(default)]
    pub experience_years: String,
}

/// Generates the self-contained JavaScript snippet to be evaluated in the target webview.
pub fn generate_autofill_script(profile: &CandidateProfile) -> String {
    let profile_json = serde_json::to_string(profile).unwrap_or_else(|_| "{}".to_string());

    format!(
        r#"(function() {{
    const profile = {profile_json};
    console.log('[Tailorbird] Executing smart autofill injection...', profile);

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
            element.classList.add('field-autofilled');
            const originalBorder = element.style.borderColor;
            const originalShadow = element.style.boxShadow;
            element.style.transition = 'all 0.3s ease';
            element.style.borderColor = '#10b981';
            element.style.boxShadow = '0 0 0 3px rgba(16, 185, 129, 0.3)';

            setTimeout(() => {{
                element.style.borderColor = originalBorder;
                element.style.boxShadow = originalShadow;
            }}, 1500);

            return true;
        }} catch (err) {{
            console.error('[Tailorbird] Failed to set native value for element:', element, err);
            return false;
        }}
    }}

    // Helper to test if element is visible and editable
    function isEditable(el) {{
        if (!el || el.disabled || el.readOnly) return false;
        const rect = el.getBoundingClientRect();
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

    // Heuristics for Full Name / First Name / Last Name
    const hasSeparateNameInputs = 
        document.querySelector('input[name*="first" i], input[id*="first" i], input[autocomplete="given-name"]') &&
        document.querySelector('input[name*="last" i], input[id*="last" i], input[autocomplete="family-name"]');

    if (hasSeparateNameInputs && firstName && lastName) {{
        tryFill('input[autocomplete="given-name"], input[name*="first" i], input[id*="first" i], input[placeholder*="first" i]', firstName);
        tryFill('input[autocomplete="family-name"], input[name*="last" i], input[id*="last" i], input[placeholder*="last" i]', lastName);
    }} else if (profile.fullName) {{
        tryFill('input[autocomplete="name"], input[name="fullName"], input[name="name"], input[name*="name" i]:not([name*="user" i]):not([name*="company" i]):not([name*="file" i]), input[id*="name" i]:not([id*="user" i]):not([id*="company" i]), input[placeholder*="full name" i]', profile.fullName);
    }}

    // Email
    if (profile.email) {{
        tryFill('input[type="email"], input[autocomplete="email"], input[name*="email" i], input[id*="email" i], input[placeholder*="email" i]', profile.email);
    }}

    // Phone
    if (profile.phone) {{
        tryFill('input[type="tel"], input[autocomplete="tel"], input[name*="phone" i], input[id*="phone" i], input[name*="mobile" i], input[placeholder*="phone" i]', profile.phone);
    }}

    // LinkedIn
    if (profile.linkedin) {{
        tryFill('input[name*="linkedin" i], input[id*="linkedin" i], input[placeholder*="linkedin" i], input[name*="urls[LinkedIn]" i]', profile.linkedin);
    }}

    // GitHub / Portfolio
    if (profile.github) {{
        tryFill('input[name*="github" i], input[id*="github" i], input[placeholder*="github" i], input[name*="urls[GitHub]" i], input[name*="portfolio" i], input[placeholder*="portfolio" i]', profile.github);
    }}

    // Years of Experience
    if (profile.experienceYears) {{
        tryFill('input[name*="experience" i], input[id*="experience" i], input[name*="years" i], input[id*="years" i], input[placeholder*="years" i]', profile.experienceYears);

        // Also check <select> for experience
        const selects = document.querySelectorAll('select[name*="experience" i], select[id*="experience" i]');
        for (const sel of selects) {{
            if (!filledElements.has(sel)) {{
                // Try matching numeric or text option
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

    console.log(`[Tailorbird] Completed autofill. Populated ${{filledCount}} fields.`);
    return filledCount;
}})();"#
    )
}
