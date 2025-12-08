/**
 * UI for iconscript SVG icon generator.
 */

(function() {
    "use strict";

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", init);
    } else {
        init();
    }

    function init() {
        const codeTextarea = document.getElementById("iconscript-code");
        const generateBtn = document.getElementById("generate-btn");
        const clearBtn = document.getElementById("clear-btn");
        const autoGenerateCheckbox = document.getElementById("auto-generate");
        const previewArea = document.getElementById("preview-area");
        const errorMessage = document.getElementById("error-message");
        const infoMessage = document.getElementById("info-message");
        const loadingIndicator = document.getElementById("loading-indicator");

        if (typeof IconScriptParser === "undefined") {
            showError("Error: bundled-parser.min.js is not loaded. Please make sure the file exists.");
            return;
        }

        let debounceTimer = null;
        const DEBOUNCE_DELAY = 500; // Milliseconds.

        generateBtn.addEventListener("click", function() {
            generateIcons();
        });

        codeTextarea.addEventListener("keydown", function(e) {
            if ((e.ctrlKey || e.metaKey) && e.key === "Enter") {
                e.preventDefault();
                generateIcons();
            }
        });

        codeTextarea.addEventListener("input", function() {
            if (autoGenerateCheckbox.checked) {
                if (debounceTimer) {
                    clearTimeout(debounceTimer);
                }
                loadingIndicator.style.display = "block";
                debounceTimer = setTimeout(function() {
                    generateIcons();
                }, DEBOUNCE_DELAY);
            } else {
                loadingIndicator.style.display = "none";
            }
        });

        autoGenerateCheckbox.addEventListener("change", function() {
            if (!this.checked && debounceTimer) {
                clearTimeout(debounceTimer);
                debounceTimer = null;
                loadingIndicator.style.display = "none";
            }
        });

        clearBtn.addEventListener("click", function() {
            codeTextarea.value = "";
            clearPreview();
            clearMessages();
            if (debounceTimer) {
                clearTimeout(debounceTimer);
                debounceTimer = null;
            }
            loadingIndicator.style.display = "none";
        });

        function generateIcons() {
            if (debounceTimer) {
                clearTimeout(debounceTimer);
                debounceTimer = null;
            }

            clearMessages();
            clearPreview();
            loadingIndicator.style.display = "block";

            const iconscriptCode = codeTextarea.value.trim();

            if (!iconscriptCode) {
                showInfo("Please enter some IconScript code.");
                loadingIndicator.style.display = "none";
                return;
            }

            try {
                const parsedIcons = IconScriptParser.parseIconsFile(iconscriptCode);

                if (!parsedIcons || parsedIcons.length === 0) {
                    showInfo("No icons found in the code.");
                    loadingIndicator.style.display = "none";
                    return;
                }
                const generator = new IconScriptParser.IconGenerator();
                let successCount = 0;
                let errorCount = 0;

                parsedIcons.forEach((icon, index) => {
                    try {
                        const svg = generator.processCommands(icon.commands);

                        if (svg) {
                            displayIcon(svg, icon.name || `icon_${index}`);
                            successCount++;
                        } else {
                            errorCount++;
                            console.warn(`Icon ${index} generated empty SVG`);
                        }
                    } catch (error) {
                        errorCount++;
                        console.error(`Error generating icon ${index}:`, error);
                        showError(`Error generating icon ${index}: ${error.message}`);
                    }
                });
                if (successCount > 0) {
                    showInfo(`Successfully generated ${successCount} icon${successCount !== 1 ? "s" : ""}.`);
                }
                if (errorCount > 0) {
                    showError(`Failed to generate ${errorCount} icon${errorCount !== 1 ? "s" : ""}.`);
                }
            } catch (error) {
                showError(`Parsing error: ${error.message}`);
                console.error("Parsing error:", error);
            } finally {
                loadingIndicator.style.display = "none";
            }
        }

        function displayIcon(svgString, iconName) {
            const iconContainer = document.createElement("div");
            iconContainer.className = "icon-container";

            const svgWrapper = document.createElement("div");
            svgWrapper.innerHTML = svgString;
            const svgElement = svgWrapper.querySelector("svg");

            if (svgElement) {
                svgElement.setAttribute("width", "64px");
                svgElement.setAttribute("height", "64px");
                svgElement.style.width = "64px";
                svgElement.style.height = "64px";

                iconContainer.appendChild(svgElement);

                if (iconName) {
                    const nameLabel = document.createElement("div");
                    nameLabel.className = "icon-name";
                    nameLabel.textContent = iconName;
                    iconContainer.appendChild(nameLabel);
                }

                previewArea.appendChild(iconContainer);
            }
        }

        function clearPreview() {
            previewArea.innerHTML = "";
        }

        function showError(message) {
            errorMessage.innerHTML = `<div class="error">${escapeHtml(message)}</div>`;
            infoMessage.innerHTML = "";
        }

        function showInfo(message) {
            infoMessage.innerHTML = `<div class="info">${escapeHtml(message)}</div>`;
            errorMessage.innerHTML = "";
        }

        function clearMessages() {
            errorMessage.innerHTML = "";
            infoMessage.innerHTML = "";
        }

        function escapeHtml(text) {
            const div = document.createElement("div");
            div.textContent = text;
            return div.innerHTML;
        }
    }
})();
