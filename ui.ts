/**
 * UI for iconscript SVG icon generator.
 */

interface ControlPoint {
    x: number;
    y: number;
    type?: "move" | "line" | "control" | "curve" | "quadratic" | "close" | "arc-start" | "arc-end";
}

interface SimplePoint {
    x: number;
    y: number;
}

(function() {
    "use strict";

    if (document.readyState === "loading") {
        document.addEventListener("DOMContentLoaded", init);
    } else {
        init();
    }

    function init(): void {
        const codeTextarea = document.getElementById("iconscript-code") as HTMLTextAreaElement;
        const generateBtn = document.getElementById("generate-btn") as HTMLButtonElement;
        const clearBtn = document.getElementById("clear-btn") as HTMLButtonElement;

        const autoGenerateCheckbox = document.getElementById("auto-generate") as HTMLInputElement;
        const showGridCheckbox = document.getElementById("show-grid") as HTMLInputElement;
        const showControlPointsCheckbox = document.getElementById("show-control-points") as HTMLInputElement;

        const previewArea = document.getElementById("preview-area") as HTMLDivElement;
        const errorMessage = document.getElementById("error-message") as HTMLDivElement;
        const infoMessage = document.getElementById("info-message") as HTMLDivElement;
        const loadingIndicator = document.getElementById("loading-indicator") as HTMLDivElement;

        if (typeof IconScriptParser === "undefined") {
            showError("Error: bundled-parser.min.js is not loaded. Please make sure the file exists.");
            return;
        }

        // Load saved content from `localStorage`.
        const STORAGE_KEY = "iconscript-code";
        const AUTO_GENERATE_KEY = "iconscript-auto-generate";
        const SHOW_GRID_KEY = "iconscript-show-grid";
        const SHOW_CONTROL_POINTS_KEY = "iconscript-show-control-points";

        const savedCode = localStorage.getItem(STORAGE_KEY);
        if (savedCode) {
            codeTextarea.value = savedCode;
        }

        // Load checkbox states from localStorage.
        const savedAutoGenerate = localStorage.getItem(AUTO_GENERATE_KEY);
        if (savedAutoGenerate !== null) {
            autoGenerateCheckbox.checked = savedAutoGenerate === "true";
        }

        const savedShowGrid = localStorage.getItem(SHOW_GRID_KEY);
        if (savedShowGrid !== null) {
            showGridCheckbox.checked = savedShowGrid === "true";
        }

        const savedShowControlPoints = localStorage.getItem(SHOW_CONTROL_POINTS_KEY);
        if (savedShowControlPoints !== null) {
            showControlPointsCheckbox.checked = savedShowControlPoints === "true";
        }

        if (autoGenerateCheckbox.checked && savedCode) {
            setTimeout(() => generateIcons(), 100);
        }

        let debounceTimer: number | null = null;
        let saveTimer: number | null = null;
        const DEBOUNCE_DELAY = 500; // Milliseconds.
        const SAVE_DELAY = 1000; // Save to localStorage after 1 second of no changes.

        generateBtn.addEventListener("click", function() {
            generateIcons();
        });

        codeTextarea.addEventListener("keydown", function(e: KeyboardEvent) {
            if ((e.ctrlKey || e.metaKey) && e.key === "Enter") {
                e.preventDefault();
                generateIcons();
            }
        });

        codeTextarea.addEventListener("input", function() {
            // Save to `localStorage` (debounced).
            if (saveTimer) {
                clearTimeout(saveTimer);
            }
            saveTimer = window.setTimeout(function() {
                localStorage.setItem(STORAGE_KEY, codeTextarea.value);
            }, SAVE_DELAY);

            if (autoGenerateCheckbox.checked) {
                if (debounceTimer) {
                    clearTimeout(debounceTimer);
                }
                loadingIndicator.style.display = "block";
                debounceTimer = window.setTimeout(function() {
                    generateIcons();
                }, DEBOUNCE_DELAY);
            } else {
                loadingIndicator.style.display = "none";
            }
        });

        autoGenerateCheckbox.addEventListener("change", function() {
            localStorage.setItem(AUTO_GENERATE_KEY, this.checked.toString());
            if (!this.checked && debounceTimer) {
                clearTimeout(debounceTimer);
                debounceTimer = null;
                loadingIndicator.style.display = "none";
            }
        });

        showGridCheckbox.addEventListener("change", function() {
            localStorage.setItem(SHOW_GRID_KEY, this.checked.toString());
            toggleGridVisibility(this.checked);
        });

        showControlPointsCheckbox.addEventListener("change", function() {
            localStorage.setItem(SHOW_CONTROL_POINTS_KEY, this.checked.toString());
            toggleControlPointsVisibility(this.checked);
        });

        clearBtn.addEventListener("click", function() {
            codeTextarea.value = "";
            localStorage.removeItem(STORAGE_KEY);
            clearPreview();
            clearMessages();
            if (debounceTimer) {
                clearTimeout(debounceTimer);
                debounceTimer = null;
            }
            if (saveTimer) {
                clearTimeout(saveTimer);
                saveTimer = null;
            }
            loadingIndicator.style.display = "none";
        });

        function generateIcons(): void {
            if (debounceTimer) {
                clearTimeout(debounceTimer);
                debounceTimer = null;
            }

            clearMessages();
            loadingIndicator.style.display = "block";

            const iconscriptCode = codeTextarea.value.trim();

            if (!iconscriptCode) {
                showInfo("Please enter some iconscript code.");
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

                // Only clear preview if parsing was successful.
                clearPreview();

                let successCount = 0;
                let errorCount = 0;

                parsedIcons.forEach((icon, index) => {
                    try {
                        const svg = icon.svg;

                        if (svg) {
                            displayIcon(svg, icon.name || `icon_${index}`);
                            successCount++;
                        } else {
                            errorCount++;
                            console.warn(`Icon ${index} generated empty SVG`);
                        }
                    } catch (error) {
                        errorCount++;
                        const errorMessage = error instanceof Error ? error.message : String(error);
                        console.error(`Error generating icon ${index}:`, error);
                        showError(`Error generating icon ${index}: ${errorMessage}`);
                    }
                });
                if (successCount > 0) {
                    showInfo(`Successfully generated ${successCount} icon${successCount !== 1 ? "s" : ""}.`);
                }
                if (errorCount > 0) {
                    showError(`Failed to generate ${errorCount} icon${errorCount !== 1 ? "s" : ""}.`);
                }
            } catch (error) {
                const errorMessage = error instanceof Error ? error.message : String(error);
                showError(`Parsing error: ${errorMessage}`);
                console.error("Parsing error:", error);
            } finally {
                loadingIndicator.style.display = "none";
            }
        }

        function extractControlPoints(pathData: string | null): SimplePoint[] {
            const points: ControlPoint[] = [];
            if (!pathData) return points.map(p => ({x: p.x, y: p.y}));

            const commands = pathData.match(/[MLHVCSQTAZmlhvcsqtaz]|[+-]?\d*\.?\d+(?:[eE][+-]?\d+)?/g) || [];
            if (commands.length === 0) return points.map(p => ({x: p.x, y: p.y}));

            let x = 0;
            let y = 0;
            let prevX = 0;
            let prevY = 0;
            let controlX = 0;
            let controlY = 0;
            let startX = 0;
            let startY = 0;
            let lastCommand: string | null = null;
            let lastIsRelative = false;

            for (let i = 0; i < commands.length; i++) {
                const cmd = commands[i];
                let command: string | null = null;
                let isRelative = false;

                if (cmd.match(/[MLHVCSQTAZmlhvcsqtaz]/)) {
                    command = cmd.toUpperCase();
                    isRelative = cmd === cmd.toLowerCase();
                    lastCommand = command;
                    lastIsRelative = isRelative;
                } else if (lastCommand && lastCommand !== "Z") {
                    // Implicit command repetition.
                    command = lastCommand;
                    isRelative = lastIsRelative;
                    i--; // Don't advance, process this number.
                } else {
                    continue;
                }

                switch (command) {
                    case "M":
                        if (i + 1 < commands.length) {
                            const newX = parseFloat(commands[++i]);
                            const newY = parseFloat(commands[++i]);
                            if (!isNaN(newX) && !isNaN(newY)) {
                                if (isRelative) {
                                    x += newX;
                                    y += newY;
                                } else {
                                    x = newX;
                                    y = newY;
                                }
                                startX = x;
                                startY = y;
                                points.push({x, y, type: "move"});
                                // After `M`, subsequent numbers are treated as
                                // `L` commands.
                                lastCommand = "L";
                            }
                        }
                        break;
                    case "L":
                        if (i + 1 < commands.length) {
                            const newX = parseFloat(commands[++i]);
                            const newY = parseFloat(commands[++i]);
                            if (!isNaN(newX) && !isNaN(newY)) {
                                if (isRelative) {
                                    x += newX;
                                    y += newY;
                                } else {
                                    x = newX;
                                    y = newY;
                                }
                                points.push({x, y, type: "line"});
                            }
                        }
                        break;
                    case "H":
                        if (i < commands.length) {
                            const newX = parseFloat(commands[++i]);
                            if (!isNaN(newX)) {
                                if (isRelative) {
                                    x += newX;
                                } else {
                                    x = newX;
                                }
                                points.push({x, y, type: "line"});
                            }
                        }
                        break;
                    case "V":
                        if (i < commands.length) {
                            const newY = parseFloat(commands[++i]);
                            if (!isNaN(newY)) {
                                if (isRelative) {
                                    y += newY;
                                } else {
                                    y = newY;
                                }
                                points.push({x, y, type: "line"});
                            }
                        }
                        break;
                    case "C":
                        if (i + 5 < commands.length) {
                            const x1 = parseFloat(commands[++i]);
                            const y1 = parseFloat(commands[++i]);
                            const x2 = parseFloat(commands[++i]);
                            const y2 = parseFloat(commands[++i]);
                            const newX = parseFloat(commands[++i]);
                            const newY = parseFloat(commands[++i]);
                            if (!isNaN(newX) && !isNaN(newY)) {
                                let absX1: number;
                                let absY1: number;
                                let absX2: number;
                                let absY2: number;
                                let absX: number;
                                let absY: number;
                                if (isRelative) {
                                    absX1 = x + x1;
                                    absY1 = y + y1;
                                    absX2 = x + x2;
                                    absY2 = y + y2;
                                    absX = x + newX;
                                    absY = y + newY;
                                    x = absX;
                                    y = absY;
                                } else {
                                    absX1 = x1;
                                    absY1 = y1;
                                    absX2 = x2;
                                    absY2 = y2;
                                    absX = newX;
                                    absY = newY;
                                    x = absX;
                                    y = absY;
                                }
                                points.push(
                                    {x: absX1, y: absY1, type: "control"},
                                    {x: absX2, y: absY2, type: "control"},
                                    {x, y, type: "curve"}
                                );
                                controlX = x;
                                controlY = y;
                            }
                        }
                        break;
                    case "S":
                        if (i + 3 < commands.length) {
                            const x2s = parseFloat(commands[++i]);
                            const y2s = parseFloat(commands[++i]);
                            const newX = parseFloat(commands[++i]);
                            const newY = parseFloat(commands[++i]);
                            if (!isNaN(newX) && !isNaN(newY)) {
                                // Calculate reflection of previous control
                                // point.
                                let prevControlX: number;
                                let prevControlY: number;
                                if (isRelative) {
                                    prevControlX = 2 * x - controlX;
                                    prevControlY = 2 * y - controlY;
                                    const absX2s = x + x2s;
                                    const absY2s = y + y2s;
                                    const absX = x + newX;
                                    const absY = y + newY;
                                    x = absX;
                                    y = absY;
                                    points.push(
                                        {x: prevControlX, y: prevControlY, type: "control"},
                                        {x: absX2s, y: absY2s, type: "control"},
                                        {x, y, type: "curve"}
                                    );
                                } else {
                                    prevControlX = 2 * x - controlX;
                                    prevControlY = 2 * y - controlY;
                                    x = newX;
                                    y = newY;
                                    points.push(
                                        {x: prevControlX, y: prevControlY, type: "control"},
                                        {x: x2s, y: y2s, type: "control"},
                                        {x, y, type: "curve"}
                                    );
                                }
                                controlX = x;
                                controlY = y;
                            }
                        }
                        break;
                    case "Q":
                        if (i + 3 < commands.length) {
                            const qx1 = parseFloat(commands[++i]);
                            const qy1 = parseFloat(commands[++i]);
                            const newX = parseFloat(commands[++i]);
                            const newY = parseFloat(commands[++i]);
                            if (!isNaN(newX) && !isNaN(newY)) {
                                let absQx1: number;
                                let absQy1: number;
                                let absX: number;
                                let absY: number;
                                if (isRelative) {
                                    absQx1 = x + qx1;
                                    absQy1 = y + qy1;
                                    absX = x + newX;
                                    absY = y + newY;
                                    x = absX;
                                    y = absY;
                                } else {
                                    absQx1 = qx1;
                                    absQy1 = qy1;
                                    absX = newX;
                                    absY = newY;
                                    x = absX;
                                    y = absY;
                                }
                                points.push(
                                    {x: absQx1, y: absQy1, type: "control"},
                                    {x, y, type: "quadratic"}
                                );
                                controlX = x;
                                controlY = y;
                            }
                        }
                        break;
                    case "T":
                        if (i + 1 < commands.length) {
                            const newX = parseFloat(commands[++i]);
                            const newY = parseFloat(commands[++i]);
                            if (!isNaN(newX) && !isNaN(newY)) {
                                // Calculate reflection of previous control
                                // point.
                                let qPrevControlX: number;
                                let qPrevControlY: number;
                                let absX: number;
                                let absY: number;
                                if (isRelative) {
                                    qPrevControlX = 2 * x - controlX;
                                    qPrevControlY = 2 * y - controlY;
                                    absX = x + newX;
                                    absY = y + newY;
                                    x = absX;
                                    y = absY;
                                } else {
                                    qPrevControlX = 2 * x - controlX;
                                    qPrevControlY = 2 * y - controlY;
                                    absX = newX;
                                    absY = newY;
                                    x = absX;
                                    y = absY;
                                }
                                points.push(
                                    {x: qPrevControlX, y: qPrevControlY, type: "control"},
                                    {x, y, type: "quadratic"}
                                );
                                controlX = x;
                                controlY = y;
                            }
                        }
                        break;
                    case "Z":
                        points.push({x, y, type: "close"});
                        x = startX;
                        y = startY;
                        lastCommand = null;
                        break;
                    case "A":
                        if (i + 6 < commands.length) {
                            const rx = parseFloat(commands[++i]);
                            const ry = parseFloat(commands[++i]);
                            const xAxisRotation = parseFloat(commands[++i]);
                            const largeArcFlag = parseFloat(commands[++i]);
                            const sweepFlag = parseFloat(commands[++i]);
                            const newX = parseFloat(commands[++i]);
                            const newY = parseFloat(commands[++i]);
                            if (!isNaN(newX) && !isNaN(newY)) {
                                const arcStartX = prevX;
                                const arcStartY = prevY;

                                // Add the start point if it's not already
                                // added.
                                if (points.length === 0 || points[points.length - 1].type !== "move") {
                                    points.push({x: arcStartX, y: arcStartY, type: "arc-start"});
                                }

                                if (isRelative) {
                                    x += newX;
                                    y += newY;
                                } else {
                                    x = newX;
                                    y = newY;
                                }
                                points.push({x, y, type: "arc-end"});
                            }
                        }
                        break;
                }

                prevX = x;
                prevY = y;
            }

            // Return points (extract just `x`, `y` for compatibility).
            return points
                .filter(p => p.type !== "control")
                .map(p => ({x: p.x, y: p.y}));
        }

        function extractPolygonPoints(svgElement: SVGElement): SimplePoint[] {

            const points: SimplePoint[] = [];

            const polygons = svgElement.querySelectorAll("polygon");
            polygons.forEach(polygon => {
                const pointsAttr = polygon.getAttribute("points");
                if (pointsAttr) {
                    // Points: `x1,y1 x2,y2 x3,y3` or `x1,y1, x2,y2, x3,y3`.
                    const coords = pointsAttr.match(/[-+]?[0-9]*\.?[0-9]+(?:[eE][-+]?[0-9]+)?/g);
                    if (coords && coords.length >= 2) {
                        for (let i = 0; i < coords.length - 1; i += 2) {
                            const x = parseFloat(coords[i]);
                            const y = parseFloat(coords[i + 1]);
                            if (!isNaN(x) && !isNaN(y)) {
                                points.push({x, y});
                            }
                        }
                    }
                }
            });

            const polylines = svgElement.querySelectorAll("polyline");
            polylines.forEach(polyline => {
                const pointsAttr = polyline.getAttribute("points");
                if (pointsAttr) {
                    const coords = pointsAttr.match(/[-+]?[0-9]*\.?[0-9]+(?:[eE][-+]?[0-9]+)?/g);
                    if (coords && coords.length >= 2) {
                        for (let i = 0; i < coords.length - 1; i += 2) {
                            const x = parseFloat(coords[i]);
                            const y = parseFloat(coords[i + 1]);
                            if (!isNaN(x) && !isNaN(y)) {
                                points.push({x, y});
                            }
                        }
                    }
                }
            });

            return points;
        }

        function displayIcon(svgString: string, iconName: string | null): void {
            const iconContainer = document.createElement("div");
            iconContainer.className = "icon-container";

            const svgWrapper = document.createElement("div");
            svgWrapper.innerHTML = svgString;
            const svgElement = svgWrapper.querySelector("svg") as SVGElement | null;

            if (svgElement) {
                svgElement.setAttribute("width", "128px");
                svgElement.setAttribute("height", "128px");
                svgElement.style.width = "128px";
                svgElement.style.height = "128px";

                // Extract control points from paths and points from
                // polygons/polylines.
                const allPoints: SimplePoint[] = [];

                // Change fill color from black to blue for all path elements.
                const pathElements = svgElement.querySelectorAll("path");
                const controlPointsVisible = showControlPointsCheckbox.checked;
                const pathOpacity = controlPointsVisible ? 0.2 : 1;
                pathElements.forEach(path => {
                    path.setAttribute("fill", `rgba(var(--fg-color), ${pathOpacity})`);
                });

                // Extract from path elements.
                const pathElement = svgElement.querySelector("path");
                if (pathElement) {
                    const pathData = pathElement.getAttribute("d");
                    const controlPoints = extractControlPoints(pathData);
                    allPoints.push(...controlPoints);
                }

                // Extract from polygon and polyline elements.
                const polygonPoints = extractPolygonPoints(svgElement);
                allPoints.push(...polygonPoints);

                // Get the viewBox, circles should use the same coordinate
                // system as the path.
                const viewBox = svgElement.getAttribute("viewBox");
                let viewBoxX = 0;
                let viewBoxY = 0;
                let viewBoxWidth = 16;
                let viewBoxHeight = 16;
                if (viewBox) {
                    const parts = viewBox.split(" ");
                    if (parts.length >= 4) {
                        viewBoxX = parseFloat(parts[0]) || 0;
                        viewBoxY = parseFloat(parts[1]) || 0;
                        viewBoxWidth = parseFloat(parts[2]) || 16;
                        viewBoxHeight = parseFloat(parts[3]) || 16;
                    }
                }

                const gridSize = 15;
                const dotSpacingX = viewBoxWidth / (gridSize + 1);
                const dotSpacingY = viewBoxHeight / (gridSize + 1);
                for (let i = 1; i <= gridSize; i++) {
                    for (let j = 1; j <= gridSize; j++) {
                        const dot = document.createElementNS(
                            "http://www.w3.org/2000/svg", "circle");
                        dot.setAttribute("cx", String(viewBoxX + i * dotSpacingX));
                        dot.setAttribute("cy", String(viewBoxY + j * dotSpacingY));
                        dot.setAttribute("r", "0.1");
                        dot.setAttribute("fill", "rgba(var(--fg-color), 0.3)");
                        dot.setAttribute("stroke", "none");
                        dot.setAttribute("class", "grid-dot");
                        if (!showGridCheckbox.checked) {
                            dot.style.display = "none";
                        }
                        svgElement.appendChild(dot);
                    }
                }

                // Remove duplicates from combined points.
                const uniquePoints: SimplePoint[] = [];
                const threshold = 0.1;
                for (const point of allPoints) {
                    let isDuplicate = false;
                    for (const existing of uniquePoints) {
                        const dx = Math.abs(point.x - existing.x);
                        const dy = Math.abs(point.y - existing.y);
                        const distance = Math.sqrt(dx * dx + dy * dy);
                        if (distance < threshold) {
                            isDuplicate = true;
                            break;
                        }
                    }
                    if (!isDuplicate) {
                        uniquePoints.push(point);
                    }
                }

                // Add dots for each point. Use the same coordinate system as
                // the path (viewBox coordinates).
                // The browser will automatically scale them when rendering.
                uniquePoints.forEach(point => {
                    const circle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
                    circle.setAttribute("cx", String(point.x));
                    circle.setAttribute("cy", String(point.y));
                    // Make the circle radius relative to viewBox (0.125 = 2px at 16px viewBox)
                    circle.setAttribute("r", "0.125");
                    circle.setAttribute("fill", "red");
                    circle.setAttribute("stroke", "none");
                    circle.setAttribute("class", "control-point");
                    if (!showControlPointsCheckbox.checked) {
                        circle.style.display = "none";
                    }
                    svgElement.appendChild(circle);
                });

                iconContainer.appendChild(svgElement);

                if (iconName) {
                    const nameLabel = document.createElement("div");
                    nameLabel.className = "icon-name";
                    nameLabel.textContent =
                        iconName.replace(/_/g, " ").toUpperCase();
                    iconContainer.appendChild(nameLabel);
                }

                previewArea.appendChild(iconContainer);
            }
        }

        function clearPreview(): void {
            previewArea.innerHTML = "";
        }

        function showError(message: string): void {
            errorMessage.innerHTML = `<div class="error">${escapeHtml(message)}</div>`;
            infoMessage.innerHTML = "";
        }

        function showInfo(message: string): void {
            infoMessage.innerHTML = `<div class="info">${escapeHtml(message)}</div>`;
            errorMessage.innerHTML = "";
        }

        function clearMessages(): void {
            errorMessage.innerHTML = "";
            infoMessage.innerHTML = "";
        }

        function escapeHtml(text: string): string {
            const div = document.createElement("div");
            div.textContent = text;
            return div.innerHTML;
        }

        function toggleGridVisibility(visible: boolean): void {
            const allSvgs = previewArea.querySelectorAll("svg");
            allSvgs.forEach(svg => {
                const gridDots = svg.querySelectorAll(".grid-dot");
                gridDots.forEach(dot => {
                    (dot as HTMLElement).style.display = visible ? "" : "none";
                });
            });
        }

        function toggleControlPointsVisibility(visible: boolean): void {
            const allSvgs = previewArea.querySelectorAll("svg");
            allSvgs.forEach(svg => {
                const controlPoints = svg.querySelectorAll(".control-point");
                controlPoints.forEach(point => {
                    (point as HTMLElement).style.display = visible ? "" : "none";
                });
                const pathElements = svg.querySelectorAll("path");
                const pathOpacity = visible ? 0.2 : 1;
                pathElements.forEach(path => {
                    path.setAttribute("fill", `rgba(var(--fg-color), ${pathOpacity})`);
                });
            });
        }
    }
})();

function updateIconStyle(): void {
    const gridDots = document.querySelectorAll(".grid-dot");
    gridDots.forEach(dot => {
        dot.setAttribute("r", "0.1");
        dot.setAttribute("stroke", "none");
    });
}
