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
/// Generates the self-contained JavaScript snippet to be evaluated in the target webview.
/// Generates the self-contained JavaScript snippet to be evaluated in the target webview.
pub fn generate_autofill_script(profile: &CandidateProfile) -> String {
    let profile_json = serde_json::to_string(profile).unwrap_or_else(|_| "{}".to_string());

    format!(
        r#"(async function() {{
    const profile = {profile_json};
    console.log('[Tailorbird] Initializing Multi-Solver Autofill Engine...', profile);

    // ========================================================================
    // 0. STYLES INJECTION
    // ========================================================================
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
            .tailorbird-scrolled-focus {{
                box-shadow: 0 0 0 4px rgba(245, 158, 11, 0.6) !important;
                outline: 2px solid #f59e0b !important;
                transition: box-shadow 0.2s ease, outline 0.2s ease !important;
            }}
            .tb-nav-btn {{
                display: inline-flex;
                align-items: center;
                justify-content: center;
                background: rgba(255, 255, 255, 0.09);
                border: 1px solid rgba(255, 255, 255, 0.16);
                color: #e5e7eb;
                border-radius: 4px;
                width: 20px;
                height: 20px;
                cursor: pointer;
                padding: 0;
                line-height: 1;
                transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease, transform 0.1s ease;
            }}
            .tb-nav-btn:hover {{
                background: rgba(245, 158, 11, 0.25);
                border-color: #f59e0b;
                color: #f59e0b;
            }}
            .tb-nav-btn:active {{
                transform: scale(0.92);
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
        (document.head || document.documentElement).appendChild(styleEl);
    }}

    document.querySelectorAll('.tailorbird-unfilled').forEach(el => el.classList.remove('tailorbird-unfilled'));
    const oldToast = document.getElementById('tailorbird-toast-notice');
    if (oldToast) oldToast.remove();

    // ========================================================================
    // 1. FORM CONTEXT & SHARED UTILITIES
    // ========================================================================
    class FormContext {{
        constructor(prof) {{
            this.profile = prof || {{}};
            this.filledElements = new Set();
            this.filledCount = 0;

            const nameParts = (this.profile.fullName || '').trim().split(/\s+/);
            this.firstName = nameParts[0] || '';
            this.lastName = nameParts.slice(1).join(' ') || '';

            this.US_STATES = {{
                'AL': 'Alabama', 'AK': 'Alaska', 'AZ': 'Arizona', 'AR': 'Arkansas', 'CA': 'California',
                'CO': 'Colorado', 'CT': 'Connecticut', 'DE': 'Delaware', 'DC': 'District of Columbia',
                'FL': 'Florida', 'GA': 'Georgia', 'HI': 'Hawaii', 'ID': 'Idaho', 'IL': 'Illinois',
                'IN': 'Indiana', 'IA': 'Iowa', 'KS': 'Kansas', 'KY': 'Kentucky', 'LA': 'Louisiana',
                'ME': 'Maine', 'MD': 'Maryland', 'MA': 'Massachusetts', 'MI': 'Michigan', 'MN': 'Minnesota',
                'MS': 'Mississippi', 'MO': 'Missouri', 'MT': 'Montana', 'NE': 'Nebraska', 'NV': 'Nevada',
                'NH': 'New Hampshire', 'NJ': 'New Jersey', 'NM': 'New Mexico', 'NY': 'New York',
                'NC': 'North Carolina', 'ND': 'North Dakota', 'OH': 'Ohio', 'OK': 'Oklahoma', 'OR': 'Oregon',
                'PA': 'Pennsylvania', 'RI': 'Rhode Island', 'SC': 'South Carolina', 'SD': 'South Dakota',
                'TN': 'Tennessee', 'TX': 'Texas', 'UT': 'Utah', 'VT': 'Vermont', 'VA': 'Virginia',
                'WA': 'Washington', 'WV': 'West Virginia', 'WI': 'Wisconsin', 'WY': 'Wyoming',
                'AB': 'Alberta', 'BC': 'British Columbia', 'MB': 'Manitoba', 'NB': 'New Brunswick',
                'NL': 'Newfoundland and Labrador', 'NS': 'Nova Scotia', 'ON': 'Ontario',
                'PE': 'Prince Edward Island', 'QC': 'Quebec', 'SK': 'Saskatchewan'
            }};

            this.MAJOR_CITIES_TO_STATE = {{
                'las vegas': 'Nevada', 'reno': 'Nevada', 'henderson': 'Nevada',
                'los angeles': 'California', 'san francisco': 'California', 'san diego': 'California',
                'san jose': 'California', 'oakland': 'California', 'sacramento': 'California',
                'austin': 'Texas', 'dallas': 'Texas', 'houston': 'Texas', 'san antonio': 'Texas',
                'seattle': 'Washington', 'new york': 'New York', 'chicago': 'Illinois',
                'denver': 'Colorado', 'boston': 'Massachusetts', 'atlanta': 'Georgia',
                'miami': 'Florida', 'orlando': 'Florida', 'tampa': 'Florida',
                'phoenix': 'Arizona', 'portland': 'Oregon', 'philadelphia': 'Pennsylvania',
                'pittsburgh': 'Pennsylvania', 'salt lake city': 'Utah', 'minneapolis': 'Minnesota',
                'detroit': 'Michigan', 'nashville': 'Tennessee', 'raleigh': 'North Carolina',
                'charlotte': 'North Carolina', 'washington': 'District of Columbia'
            }};
        }}

        markFilled(el) {{
            if (!el || this.filledElements.has(el)) return false;
            this.filledElements.add(el);
            this.filledCount++;
            el.classList.remove('tailorbird-unfilled');
            el.classList.add('tailorbird-filled');
            setTimeout(() => {{
                el.classList.remove('tailorbird-filled');
            }}, 2000);
            return true;
        }}

        isEditable(el) {{
            if (!el || el.disabled || el.readOnly) return false;
            return el.type !== 'hidden';
        }}

        resolveStateFromLocation(loc) {{
            if (!loc) return '';
            const s = loc.trim().toLowerCase();
            for (const [abbr, fullName] of Object.entries(this.US_STATES)) {{
                if (s.includes(fullName.toLowerCase()) || new RegExp('\\b' + abbr + '\\b', 'i').test(loc)) {{
                    return fullName;
                }}
            }}
            for (const [city, state] of Object.entries(this.MAJOR_CITIES_TO_STATE)) {{
                if (s.includes(city)) return state;
            }}
            return '';
        }}

        resolveCountryFromLocation(loc) {{
            if (!loc) return 'United States';
            const s = loc.trim().toLowerCase();
            if (s.includes('canada') || s.includes('ontario') || s.includes('quebec') || s.includes('british columbia')) {{
                return 'Canada';
            }}
            if (s.includes('uk') || s.includes('united kingdom') || s.includes('england')) return 'United Kingdom';
            return 'United States';
        }}

        resolveLabel(el) {{
            if (!el) return '';
            let label = '';

            if (el.id) {{
                const l1 = document.querySelector(`label[for="${{CSS.escape(el.id)}}"]`);
                if (l1) label += ' ' + l1.textContent;
                const l2 = document.getElementById(el.id + '-label');
                if (l2) label += ' ' + l2.textContent;
            }}

            const labelledBy = el.getAttribute('aria-labelledby');
            if (labelledBy) {{
                labelledBy.split(/\s+/).forEach(id => {{
                    const target = document.getElementById(id);
                    if (target) label += ' ' + target.textContent;
                }});
            }}

            const ariaLabel = el.getAttribute('aria-label');
            if (ariaLabel) label += ' ' + ariaLabel;
            if (el.placeholder) label += ' ' + el.placeholder;
            if (el.name) label += ' ' + el.name;
            if (el.id) label += ' ' + el.id;

            let curr = el.parentElement;
            for (let i = 0; i < 6 && curr && curr !== document.body; i++) {{
                const cls = (curr.className || '').toString().toLowerCase();
                const tag = (curr.tagName || '').toLowerCase();
                if (
                    cls.includes('field') || cls.includes('question') || cls.includes('input') ||
                    cls.includes('group') || cls.includes('row') || tag === 'fieldset' || tag === 'li'
                ) {{
                    const innerLabels = curr.querySelectorAll('label, legend, .label, [class*="label"], [class*="title"], h1, h2, h3, h4, h5, h6');
                    innerLabels.forEach(lbl => {{
                        label += ' ' + lbl.textContent;
                    }});
                    break;
                }}
                curr = curr.parentElement;
            }}

            return label.replace(/[\*\:\?]/g, ' ').replace(/\s+/g, ' ').trim().toLowerCase();
        }}

        resolveSemanticTarget(labelText) {{
            const lbl = (labelText || '').toLowerCase();

            // High-priority exact identity
            if (/preferred.*name|nickname/i.test(lbl)) {{
                return {{ field: 'preferredName', value: this.firstName || this.profile.fullName }};
            }}
            if (/pronunciation|phonetic/i.test(lbl)) {{
                return {{ field: 'namePronunciation', value: this.firstName || this.profile.fullName }};
            }}
            if (/first.*name|given.*name/i.test(lbl)) {{
                return {{ field: 'firstName', value: this.firstName }};
            }}
            if (/last.*name|family.*name|surname/i.test(lbl)) {{
                return {{ field: 'lastName', value: this.lastName }};
            }}
            if (/\bfull.*name\b|^name$/i.test(lbl) && !/company|file|user|user_name/i.test(lbl)) {{
                return {{ field: 'fullName', value: this.profile.fullName }};
            }}

            // Contact & Links
            if (/\bemail\b/i.test(lbl)) return {{ field: 'email', value: this.profile.email }};
            if (/\bphone\b|\bmobile\b|\bcell\b|\btelephone\b/i.test(lbl)) return {{ field: 'phone', value: this.profile.phone }};
            if (/linkedin/i.test(lbl)) return {{ field: 'linkedin', value: this.profile.linkedin }};
            if (/github/i.test(lbl)) return {{ field: 'github', value: this.profile.github }};
            if (/twitter|\bx\b.*handle|\bx\b.*profile/i.test(lbl)) {{
                const twVal = this.profile.github ? `https://x.com/${{this.profile.github.split('/').filter(Boolean).pop()}}` : (this.profile.portfolioUrl || '');
                return {{ field: 'twitter', value: twVal }};
            }}
            if (/portfolio|personal.*website|personal.*url|website/i.test(lbl)) {{
                return {{ field: 'portfolio', value: this.profile.portfolioUrl }};
            }}

            // Location
            if (/\bcountry\b|nationality/i.test(lbl)) {{
                return {{ field: 'country', value: this.resolveCountryFromLocation(this.profile.location) }};
            }}
            if (/which.*state|\bstate\b.*province|province/i.test(lbl) && !/united states/i.test(lbl)) {{
                return {{ field: 'state', value: this.resolveStateFromLocation(this.profile.location) }};
            }}
            if (/location|\bcity\b|\baddress\b/i.test(lbl) && !/ethnic/i.test(lbl)) {{
                return {{ field: 'location', value: this.profile.location }};
            }}

            // Employment & Sponsorship
            if (/current.*company|recent.*company|\bcurrent.*employer\b|\brecent.*employer\b|^company$|^current company$/i.test(lbl)) {{
                return {{ field: 'currentCompany', value: this.profile.currentCompany }};
            }}
            if (/sponsorship|require.*visa|require.*immigration|work.*authorization.*future/i.test(lbl)) {{
                return {{ field: 'sponsorship', value: 'No' }};
            }}
            if (/experience|years/i.test(lbl)) {{
                return {{ field: 'experienceYears', value: this.profile.experienceYears }};
            }}
            if (/salary|compensation|pay/i.test(lbl)) {{
                const sal = (this.profile.salaryMin && this.profile.salaryMax) ? `${{this.profile.salaryMin}} - ${{this.profile.salaryMax}}` : (this.profile.salaryMin || this.profile.salaryMax || '');
                return {{ field: 'salary', value: sal }};
            }}

            // Demographics & EEO
            if (/pronoun/i.test(lbl)) return {{ field: 'pronouns', value: this.profile.pronouns }};
            if (/gender|sex\b/i.test(lbl)) return {{ field: 'gender', value: this.profile.gender }};
            if (/hispanic|latino/i.test(lbl)) {{
                const isHisp = (this.profile.race || '').toLowerCase().includes('hispanic');
                return {{ field: 'hispanic', value: isHisp ? 'Yes' : 'No' }};
            }}
            if (/race|ethnic/i.test(lbl)) return {{ field: 'race', value: this.profile.race }};
            if (/veteran|military/i.test(lbl)) return {{ field: 'veteranStatus', value: this.profile.veteranStatus }};
            if (/disabilit/i.test(lbl)) return {{ field: 'disabilityStatus', value: this.profile.disabilityStatus }};

            return null;
        }}

        isOptionMatch(targetVal, optVal, optText) {{
            if (!targetVal) return false;
            const t = targetVal.toLowerCase().trim();
            const v = (optVal !== undefined && optVal !== null) ? String(optVal).toLowerCase().trim() : '';
            const txt = (optText !== undefined && optText !== null) ? String(optText).toLowerCase().trim() : '';

            if (v === t || txt === t) return true;

            // Gender
            if (t === 'male') {{
                return (txt === 'male' || v === 'male' || txt === 'man' || v === 'man' || txt.startsWith('male ') || txt.startsWith('male/'));
            }}
            if (t === 'female') {{
                return (txt === 'female' || v === 'female' || txt === 'woman' || v === 'woman' || txt.startsWith('female ') || txt.startsWith('female/'));
            }}
            if (t === 'non-binary') {{
                return (txt.includes('non-binary') || v.includes('non-binary'));
            }}

            // Race / Ethnicity
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

            // Veteran
            if (t.includes('not a protected veteran') || t.includes('not a veteran')) {{
                if (txt.includes('not a protected veteran') || txt.includes('not a veteran') || v.includes('not a protected veteran')) return true;
            }}
            if (t.includes('one or more') || (t.includes('protected veteran') && !t.includes('not'))) {{
                if ((txt.includes('one or more') || txt.includes('protected veteran')) && !txt.includes('not a protected veteran') && !txt.includes('not a veteran')) return true;
            }}

            // Disability
            const isDecline = t.includes('wish to answer') || t.includes('prefer not') || t.includes('decline') || t.includes('disclose');
            if (isDecline) {{
                if (txt.includes('prefer not') || txt.includes('wish to answer') || txt.includes('decline') || txt.includes('disclose') || v.includes('prefer not')) return true;
            }}
            if (!isDecline && (t === 'yes' || t.includes('have a disability'))) {{
                if (txt === 'yes' || v === 'yes' || (txt.includes('yes') && !txt.includes('no'))) return true;
            }}
            if (!isDecline && (t === 'no' || t.includes('do not have a disability') || t.includes("don't have a disability"))) {{
                if (txt === 'no' || v === 'no' || (txt.includes('no') && !txt.includes('yes'))) return true;
            }}

            // Pronouns
            if (t.includes('he/him')) {{
                if (txt.includes('he/him') || v.includes('he/him') || txt.includes('he / him')) return true;
            }}
            if (t.includes('she/her')) {{
                if (txt.includes('she/her') || v.includes('she/her') || txt.includes('she / her')) return true;
            }}
            if (t.includes('they/them')) {{
                if (txt.includes('they/them') || v.includes('they/them') || txt.includes('they / them')) return true;
            }}

            // Sponsorship & Hispanic Yes/No
            if (t === 'no' && (txt === 'no' || v === 'no' || txt.startsWith('no '))) return true;
            if (t === 'yes' && (txt === 'yes' || v === 'yes' || txt.startsWith('yes '))) return true;

            // Country
            if (t === 'united states' && (txt.includes('united states') || txt === 'usa' || v === 'us' || v === 'usa')) return true;

            // General fallback
            if (v && v.includes(t)) return true;
            if (txt && txt.includes(t)) return true;

            return false;
        }}

        setNativeValue(element, value) {{
            if (!element || value === undefined || value === null) return false;
            try {{
                let prototype = Object.getPrototypeOf(element);
                let descriptor = Object.getOwnPropertyDescriptor(prototype, 'value');
                while (prototype && !descriptor) {{
                    prototype = Object.getPrototypeOf(prototype);
                    if (prototype) descriptor = Object.getOwnPropertyDescriptor(prototype, 'value');
                }}

                if (descriptor && descriptor.set) {{
                    descriptor.set.call(element, value);
                }} else {{
                    element.value = value;
                }}

                element.dispatchEvent(new Event('focus', {{ bubbles: true, cancelable: true }}));
                element.dispatchEvent(new Event('input', {{ bubbles: true, cancelable: true }}));
                element.dispatchEvent(new Event('change', {{ bubbles: true, cancelable: true }}));
                element.dispatchEvent(new Event('blur', {{ bubbles: true, cancelable: true }}));

                this.markFilled(element);
                return true;
            }} catch (err) {{
                console.error('[Tailorbird] Error setting native value:', err);
                return false;
            }}
        }}
    }}

    // ========================================================================
    // 2. SOLVER: REACT-SELECT & ARIA COMBOBOX SOLVER
    // ========================================================================
    class ReactSelectSolver {{
        static async selectOption(inputEl, targetVal, allowedOptions, context) {{
            if (!inputEl || !targetVal) return false;

            // Strategy 1: React Fiber / memoizedProps inspection
            try {{
                let curr = inputEl;
                let fiber = null;
                for (let i = 0; i < 6 && curr; i++) {{
                    const key = Object.keys(curr).find(k => k.startsWith('__reactFiber$') || k.startsWith('__reactInternalInstance$'));
                    if (key && curr[key]) {{
                        fiber = curr[key];
                        break;
                    }}
                    curr = curr.parentElement;
                }}

                let traveler = fiber;
                while (traveler) {{
                    const props = traveler.memoizedProps;
                    if (props) {{
                        // CRITICAL: Must be the Select component itself or child with selectProps!
                        // Do NOT call onChange on DOM <input> element's own native event props!
                        const selectProps = props.selectProps || (Array.isArray(props.options) ? props : null);
                        if (selectProps && typeof selectProps.onChange === 'function') {{
                            const opts = (Array.isArray(selectProps.options) && selectProps.options.length > 0)
                                ? selectProps.options
                                : allowedOptions;

                            if (Array.isArray(opts)) {{
                                const matchedOpt = opts.find(opt => {{
                                    const oLabel = opt.label || opt.name || opt.text || '';
                                    const oVal = opt.value !== undefined ? opt.value : (opt.id !== undefined ? opt.id : '');
                                    return context.isOptionMatch(targetVal, String(oVal), String(oLabel));
                                }});

                                if (matchedOpt) {{
                                    const valToPass = selectProps.isMulti ? [matchedOpt] : matchedOpt;
                                    selectProps.onChange(valToPass, {{ action: 'select-option', option: matchedOpt }});
                                    if (typeof selectProps.onBlur === 'function') {{
                                        selectProps.onBlur(new FocusEvent('blur'));
                                    }}
                                    ReactSelectSolver.invalidateHiddenInputs(inputEl, matchedOpt.value || matchedOpt.id || matchedOpt.label, context);
                                    context.markFilled(inputEl);
                                    return true;
                                }}
                            }}
                        }}
                    }}
                    traveler = traveler.return;
                }}
            }} catch (err) {{
                console.warn('[Tailorbird] React-Select fiber attempt error:', err);
            }}

            // Strategy 2: Asynchronous DOM Simulation with guaranteed menu closure
            try {{
                const control = inputEl.closest('.select__control') || inputEl.parentElement;
                inputEl.focus();

                // 1. Open dropdown
                if (control) {{
                    control.dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true, cancelable: true, view: window }}));
                    control.dispatchEvent(new MouseEvent('mouseup', {{ bubbles: true, cancelable: true, view: window }}));
                    control.click();
                }}
                inputEl.dispatchEvent(new KeyboardEvent('keydown', {{ key: 'ArrowDown', code: 'ArrowDown', keyCode: 40, bubbles: true }}));

                // 2. Wait for React to render the menu asynchronously (prevents race condition)
                await new Promise(r => setTimeout(r, 80));

                // 3. Search options in mounted menu
                let didSelect = false;
                const renderedOptions = Array.from(document.querySelectorAll('.select__option, [id*="react-select"][role="option"], [class*="-option"]'));
                for (const opt of renderedOptions) {{
                    if (context.isOptionMatch(targetVal, opt.getAttribute('data-value'), opt.textContent)) {{
                        opt.dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true, cancelable: true, view: window }}));
                        opt.dispatchEvent(new MouseEvent('mouseup', {{ bubbles: true, cancelable: true, view: window }}));
                        opt.click();
                        didSelect = true;
                        ReactSelectSolver.invalidateHiddenInputs(inputEl, targetVal, context);
                        context.markFilled(inputEl);
                        await new Promise(r => setTimeout(r, 40));
                        break;
                    }}
                }}

                // If not found in default view, try typing filter
                if (!didSelect) {{
                    inputEl.value = targetVal;
                    inputEl.dispatchEvent(new Event('input', {{ bubbles: true }}));
                    await new Promise(r => setTimeout(r, 60));

                    const filtered = Array.from(document.querySelectorAll('.select__option, [id*="react-select"][role="option"], [class*="-option"]'));
                    if (filtered.length > 0) {{
                        filtered[0].dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true, cancelable: true, view: window }}));
                        filtered[0].dispatchEvent(new MouseEvent('mouseup', {{ bubbles: true, cancelable: true, view: window }}));
                        filtered[0].click();
                        didSelect = true;
                        ReactSelectSolver.invalidateHiddenInputs(inputEl, targetVal, context);
                        context.markFilled(inputEl);
                        await new Promise(r => setTimeout(r, 40));
                    }}
                }}

                // 4. GUARANTEE: NEVER leave any menu open!
                const menuEl = document.querySelector('.select__menu, [class*="-menu"]');
                if (menuEl) {{
                    inputEl.dispatchEvent(new KeyboardEvent('keydown', {{ key: 'Escape', code: 'Escape', keyCode: 27, bubbles: true }}));
                    if (control) {{
                        control.dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true, cancelable: true, view: window }}));
                    }}
                    inputEl.blur();
                    await new Promise(r => setTimeout(r, 30));
                }}

                return didSelect;
            }} catch (err) {{
                console.warn('[Tailorbird] React-Select DOM simulation error:', err);
                inputEl.dispatchEvent(new KeyboardEvent('keydown', {{ key: 'Escape', code: 'Escape', keyCode: 27, bubbles: true }}));
                inputEl.blur();
            }}

            return false;
        }}

        static invalidateHiddenInputs(inputEl, value, context) {{
            const wrapper = inputEl.closest('.field-wrapper, .select-wrapper, .input-wrapper') || inputEl.parentElement;
            if (wrapper) {{
                const hiddenInputs = wrapper.querySelectorAll('input.remix-css-1a0ro4n-requiredInput, input[tabindex="-1"][required], input[type="hidden"]');
                hiddenInputs.forEach(h => {{
                    context.setNativeValue(h, value || 'selected');
                }});
            }}
        }}

        static async solve(context) {{
            const comboboxInputs = Array.from(document.querySelectorAll('input.select__input, input[role="combobox"]'));
            for (const input of comboboxInputs) {{
                if (context.filledElements.has(input)) continue;
                const labelText = context.resolveLabel(input);
                const semantic = context.resolveSemanticTarget(labelText);
                if (semantic && semantic.value) {{
                    await ReactSelectSolver.selectOption(input, semantic.value, null, context);
                }}
            }}
        }}
    }}

    // ========================================================================
    // 3. SOLVER: GREENHOUSE ATS SPECIALIZED SOLVER
    // ========================================================================
    class GreenhouseAtsSolver {{
        static canSolve() {{
            return !!(window.__remixContext || document.getElementById('application-form') || location.hostname.includes('greenhouse.io'));
        }}

        static async solve(context) {{
            if (!window.__remixContext || !window.__remixContext.state || !window.__remixContext.state.loaderData) {{
                return false;
            }}

            const loaderData = window.__remixContext.state.loaderData;
            const jobPostRouteKey = Object.keys(loaderData).find(k => loaderData[k] && loaderData[k].jobPost);
            if (!jobPostRouteKey) return false;

            const jobPost = loaderData[jobPostRouteKey].jobPost;
            console.log('[Tailorbird] Running GreenhouseAtsSolver with job schema questions:', (jobPost.questions || []).length);

            // 1. Standard Fields (first_name, last_name, email, phone, country)
            const standardMap = [
                {{ id: 'first_name', val: context.firstName }},
                {{ id: 'last_name', val: context.lastName }},
                {{ id: 'email', val: context.profile.email }},
                {{ id: 'phone', val: context.profile.phone }},
                {{ id: 'country', val: context.resolveCountryFromLocation(context.profile.location) }}
            ];

            for (const item of standardMap) {{
                const el = document.getElementById(item.id);
                if (el && item.val && !context.filledElements.has(el)) {{
                    if (el.getAttribute('role') === 'combobox' || el.classList.contains('select__input')) {{
                        await ReactSelectSolver.selectOption(el, item.val, null, context);
                    }} else {{
                        context.setNativeValue(el, item.val);
                    }}
                }}
            }}

            // 2. Custom Job Post Questions
            if (Array.isArray(jobPost.questions)) {{
                for (const q of jobPost.questions) {{
                    if (!q.fields || !Array.isArray(q.fields)) continue;
                    const semantic = context.resolveSemanticTarget(q.label || '');
                    if (!semantic || !semantic.value) continue;

                    for (const f of q.fields) {{
                        const el = document.getElementById(f.name);
                        if (!el || context.filledElements.has(el)) continue;

                        if (f.type === 'multi_value_single_select' || el.getAttribute('role') === 'combobox' || el.classList.contains('select__input')) {{
                            await ReactSelectSolver.selectOption(el, semantic.value, f.values, context);
                        }} else if (f.type === 'input_text' || f.type === 'textarea') {{
                            context.setNativeValue(el, semantic.value);
                        }}
                    }}
                }}
            }}

            // 3. EEOC Sections
            if (Array.isArray(jobPost.eeoc_sections)) {{
                for (const sec of jobPost.eeoc_sections) {{
                    if (!Array.isArray(sec.questions)) continue;
                    for (const q of sec.questions) {{
                        if (!q.fields || !Array.isArray(q.fields)) continue;
                        const semantic = context.resolveSemanticTarget(q.label || '');
                        if (!semantic || !semantic.value) continue;

                        for (const f of q.fields) {{
                            const el = document.getElementById(f.name);
                            if (!el || context.filledElements.has(el)) continue;
                            await ReactSelectSolver.selectOption(el, semantic.value, f.values, context);
                        }}
                    }}
                }}
            }}

            // 4. Standard EEOC specific field IDs
            const eeocFields = [
                {{ id: 'gender', target: context.profile.gender }},
                {{ id: 'hispanic_ethnicity', target: (context.profile.race || '').toLowerCase().includes('hispanic') ? 'Yes' : 'No' }},
                {{ id: 'veteran_status', target: context.profile.veteranStatus }},
                {{ id: 'race', target: context.profile.race }}
            ];

            for (const item of eeocFields) {{
                const el = document.getElementById(item.id);
                if (el && item.target && !context.filledElements.has(el)) {{
                    await ReactSelectSolver.selectOption(el, item.target, null, context);
                }}
            }}

            // 5. Demographic Questions Survey
            if (jobPost.demographic_questions && Array.isArray(jobPost.demographic_questions.questions)) {{
                for (const q of jobPost.demographic_questions.questions) {{
                    const el = document.getElementById(String(q.id));
                    if (!el || context.filledElements.has(el)) continue;

                    const semantic = context.resolveSemanticTarget(q.name || '');
                    if (semantic && semantic.value) {{
                        const opts = Array.isArray(q.answer_options) ? q.answer_options.map(o => ({{ label: o.name, value: o.id }})) : null;
                        await ReactSelectSolver.selectOption(el, semantic.value, opts, context);
                    }}
                }}
            }}

            return true;
        }}
    }}

    // ========================================================================
    // 4. SOLVER: NATIVE INPUT SOLVER
    // ========================================================================
    class NativeInputSolver {{
        static solve(context) {{
            const inputs = Array.from(document.querySelectorAll('input:not([type="hidden"]):not([type="submit"]):not([type="button"]):not([type="reset"]):not([type="checkbox"]):not([type="radio"]), textarea'));
            for (const el of inputs) {{
                if (context.filledElements.has(el) || el.classList.contains('select__input') || el.getAttribute('role') === 'combobox') continue;
                if (!context.isEditable(el)) continue;

                const labelText = context.resolveLabel(el);
                const semantic = context.resolveSemanticTarget(labelText);
                if (semantic && semantic.value) {{
                    context.setNativeValue(el, semantic.value);
                }}
            }}
        }}
    }}

    // ========================================================================
    // 5. SOLVER: NATIVE SELECT & RADIO/CHECKBOX SOLVER
    // ========================================================================
    class NativeSelectSolver {{
        static solve(context) {{
            // Select dropdowns
            const selects = Array.from(document.querySelectorAll('select'));
            for (const sel of selects) {{
                if (context.filledElements.has(sel) || !context.isEditable(sel)) continue;
                const labelText = context.resolveLabel(sel);
                const semantic = context.resolveSemanticTarget(labelText);
                if (semantic && semantic.value) {{
                    for (const opt of sel.options) {{
                        if (context.isOptionMatch(semantic.value, opt.value, opt.text)) {{
                            sel.value = opt.value;
                            sel.dispatchEvent(new Event('input', {{ bubbles: true }}));
                            sel.dispatchEvent(new Event('change', {{ bubbles: true }}));
                            context.markFilled(sel);
                            break;
                        }}
                    }}
                }}
            }}

            // Radio / Checkbox
            const checkables = Array.from(document.querySelectorAll('input[type="radio"], input[type="checkbox"]'));
            for (const input of checkables) {{
                if (context.filledElements.has(input) || input.checked) continue;
                const labelText = context.resolveLabel(input);
                const semantic = context.resolveSemanticTarget(labelText);
                if (semantic && semantic.value) {{
                    const parentText = input.closest('label')?.textContent || input.parentElement?.textContent || '';
                    if (context.isOptionMatch(semantic.value, input.value, parentText)) {{
                        input.checked = true;
                        input.dispatchEvent(new Event('input', {{ bubbles: true }}));
                        input.dispatchEvent(new Event('change', {{ bubbles: true }}));
                        context.markFilled(input);
                    }}
                }}
            }}
        }}
    }}

    // ========================================================================
    // 6. MASTER ORCHESTRATOR
    // ========================================================================
    const context = new FormContext(profile);

    // Run specialized ATS solver first if applicable
    if (GreenhouseAtsSolver.canSolve()) {{
        await GreenhouseAtsSolver.solve(context);
    }}

    // Run React-Select solver for any remaining custom comboboxes
    await ReactSelectSolver.solve(context);

    // Run Native Input & Select solvers
    NativeInputSolver.solve(context);
    NativeSelectSolver.solve(context);

    // ========================================================================
    // 7. UNFILLED HIGHLIGHTS & TOAST NOTIFICATION
    // ========================================================================
    const allFormControls = Array.from(document.querySelectorAll('input, select, textarea'));
    let unfilledCount = 0;

    const checkedGroups = new Set();
    allFormControls.forEach(el => {{
        if ((el.type === 'radio' || el.type === 'checkbox') && el.name && el.checked) {{
            checkedGroups.add(el.name);
        }}
    }});

    allFormControls.forEach(el => {{
        if (el.type === 'hidden' || el.type === 'submit' || el.type === 'button' || el.type === 'reset') return;
        if (el.type === 'search' || el.getAttribute('role') === 'search') return;
        if (el.classList.contains('remix-css-1a0ro4n-requiredInput') || el.tabIndex === -1 && el.getAttribute('aria-hidden') === 'true') return;
        if (el.type !== 'file' && el.offsetParent === null && el.getClientRects().length === 0) return;

        let isUnfilled = false;
        if (el.type === 'radio' || el.type === 'checkbox') {{
            if (el.name) {{
                if (!checkedGroups.has(el.name)) isUnfilled = true;
            }} else if (!el.checked) {{
                isUnfilled = true;
            }}
        }} else if (el.tagName === 'SELECT') {{
            if (el.selectedIndex <= 0 || !el.value || el.value.trim() === '') isUnfilled = true;
        }} else if (el.classList.contains('select__input') || el.getAttribute('role') === 'combobox') {{
            const wrapper = el.closest('.select__control, .field-wrapper, .select-wrapper');
            const hasValuePill = wrapper && wrapper.querySelector('.select__single-value, .select__multi-value, [class*="singleValue"], [class*="multiValue"]');
            if (!hasValuePill) isUnfilled = true;
        }} else {{
            if (!el.value || el.value.trim() === '') isUnfilled = true;
        }}

        if (isUnfilled && !context.filledElements.has(el)) {{
            el.classList.add('tailorbird-unfilled');
            unfilledCount++;

            if (!el._tailorbirdListenerAttached) {{
                el._tailorbirdListenerAttached = true;
                const clearHighlight = () => {{
                    let nowFilled = false;
                    if (el.type === 'radio' || el.type === 'checkbox') {{
                        if (el.name) {{
                            const anyChecked = document.querySelector(`input[name="${{CSS.escape(el.name)}}"]:checked`);
                            if (anyChecked) {{
                                document.querySelectorAll(`input[name="${{CSS.escape(el.name)}}"]`).forEach(inp => inp.classList.remove('tailorbird-unfilled'));
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

    let currentUnfilledIndex = -1;

    function getUnfilledTargets() {{
        const elements = Array.from(document.querySelectorAll('.tailorbird-unfilled'));
        const seenGroups = new Set();
        const targets = [];
        for (const el of elements) {{
            if (el.type === 'radio' || el.type === 'checkbox') {{
                if (el.name) {{
                    if (seenGroups.has(el.name)) continue;
                    seenGroups.add(el.name);
                }}
            }}
            targets.push(el);
        }}
        return targets;
    }}

    function navigateUnfilled(direction) {{
        const targets = getUnfilledTargets();
        if (targets.length === 0) return;

        if (direction === 'next') {{
            currentUnfilledIndex = (currentUnfilledIndex + 1) % targets.length;
        }} else {{
            currentUnfilledIndex = (currentUnfilledIndex - 1 + targets.length) % targets.length;
        }}

        const target = targets[currentUnfilledIndex];
        const scrollTarget = target.closest('.field-wrapper, .select, .input-wrapper, fieldset, li') || target;
        scrollTarget.scrollIntoView({{ behavior: 'smooth', block: 'center' }});

        try {{
            target.focus({{ preventScroll: true }});
        }} catch (e) {{}}

        const highlightEl = target.closest('.select__control') || target;
        highlightEl.classList.add('tailorbird-scrolled-focus');
        setTimeout(() => {{
            highlightEl.classList.remove('tailorbird-scrolled-focus');
        }}, 1500);
    }}

    function updateToast() {{
        const toast = document.getElementById('tailorbird-toast-notice');
        if (!toast) return;

        const targets = getUnfilledTargets();
        const remainingCount = targets.length;

        if (remainingCount > 0) {{
            const countText = toast.querySelector('.tb-unfilled-text');
            if (countText) {{
                countText.innerHTML = `⚠️ <strong>${{remainingCount}}</strong> remaining`;
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

    const toast = document.createElement('div');
    toast.id = 'tailorbird-toast-notice';
    if (unfilledCount > 0) {{
        toast.innerHTML = `
            <span>⚡ <strong>${{context.filledCount}}</strong> field${{context.filledCount === 1 ? '' : 's'}} autofilled</span>
            <span style="color: #64748b;">•</span>
            <span id="tailorbird-unfilled-count" style="color: #f59e0b; display: inline-flex; align-items: center; gap: 6px;">
                <span class="tb-unfilled-text">⚠️ <strong>${{unfilledCount}}</strong> remaining</span>
                <span class="tb-nav-group" style="display: inline-flex; gap: 3px; align-items: center; margin-left: 2px;">
                    <button class="tb-nav-btn" id="tb-nav-prev" type="button" title="Scroll to previous unfilled field" aria-label="Previous unfilled field">
                        <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="18 15 12 9 6 15"></polyline></svg>
                    </button>
                    <button class="tb-nav-btn" id="tb-nav-next" type="button" title="Scroll to next unfilled field" aria-label="Next unfilled field">
                        <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"></polyline></svg>
                    </button>
                </span>
            </span>
            <button style="background:none; border:none; color:#9ca3af; font-size:13px; cursor:pointer; margin-left:6px; padding:0 4px;" onclick="this.parentElement.remove()">✕</button>
        `;

        const prevBtn = toast.querySelector('#tb-nav-prev');
        if (prevBtn) prevBtn.addEventListener('click', (e) => {{ e.stopPropagation(); navigateUnfilled('prev'); }});

        const nextBtn = toast.querySelector('#tb-nav-next');
        if (nextBtn) nextBtn.addEventListener('click', (e) => {{ e.stopPropagation(); navigateUnfilled('next'); }});
    }} else {{
        toast.style.borderColor = '#10b981';
        toast.innerHTML = `
            <span>⚡ <strong>${{context.filledCount}}</strong> field${{context.filledCount === 1 ? '' : 's'}} autofilled</span>
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

    console.log(`[Tailorbird] Completed autofill: ${{context.filledCount}} populated, ${{unfilledCount}} unfilled highlighted.`);
    return context.filledCount;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_autofill_script_contains_solvers() {
        let profile = CandidateProfile {
            full_name: "Joel Stransky".to_string(),
            pronouns: "He/him".to_string(),
            email: "stranskydesign@gmail.com".to_string(),
            phone: "(915) 474-2746".to_string(),
            location: "Las Vegas".to_string(),
            current_company: "College Loan Corporation".to_string(),
            linkedin: "https://www.linkedin.com/in/joelstransky/".to_string(),
            github: "https://github.com/joelstransky".to_string(),
            portfolio_url: "https://joelstransky.netlify.app/".to_string(),
            experience_years: "15".to_string(),
            gender: "Male".to_string(),
            race: "White".to_string(),
            veteran_status: "I am not a protected veteran".to_string(),
            disability_status: "No, I don't have a disability".to_string(),
            ..Default::default()
        };

        let script = generate_autofill_script(&profile);
        assert!(script.contains("FormContext"));
        assert!(script.contains("ReactSelectSolver"));
        assert!(script.contains("GreenhouseAtsSolver"));
        assert!(script.contains("NativeInputSolver"));
        assert!(script.contains("NativeSelectSolver"));
        assert!(script.contains("Joel Stransky"));
        assert!(script.contains("College Loan Corporation"));
        assert!(script.contains("Las Vegas"));
    }
}

