const { JSDOM } = require('jsdom');
const { NodeHtmlMarkdown } = require('node-html-markdown');
const fs = require("fs/promises");
const { createWriteStream } = require("fs");
const fetch = require("node-fetch");
const { pipeline } = require('stream/promises');
const replace = require('replace-in-file');

const year = (new Date()).getFullYear();
const day = (new Date()).getDate();
let padded = String(day).padStart(2, '0');
let outdir = `challenges/day${padded}`;

async function setup() {
    console.log("=== SETUP ===");

    // create rust project
    await fs.cp("challenges/day00", outdir, {
        recursive: true,
    });
    await fs.unlink(`${outdir}/input.txt`);

    await replace({
        files: `${outdir}/**/*`,
        from: /(day)00/gi,
        to: (...args) => `${args[1]}${padded}`,
    });

    // download problem input
    const input = await fetch(`https://adventofcode.com/${year}/day/${day}/input`, {
        headers: { 'Cookie': `session=${process.env.AOC_SESSION}` }
    });
    await pipeline(input.body, createWriteStream(`${outdir}/input.txt`));

    await get_description();
}

async function get_description() {
    console.log("=== UPDATE ===");

    // download description and create readme
    const input = await fetch(`https://adventofcode.com/${year}/day/${day}`, {
        headers: { 'Cookie': `session=${process.env.AOC_SESSION}` }
    });
    const dom = new JSDOM(await input.text());

    let md = "";
    dom.window.document.querySelectorAll(".day-desc").forEach(e => md += NodeHtmlMarkdown.translate(e.innerHTML, { emDelimiter: "**" }) + "\n\n");

    // fix numerical highlighting
    md = md.replace(/\`\*\*([0-9]+)\*\*\`/g, "**`$1`**")

    await fs.writeFile(`${outdir}/README.md`, md);
}

if (require('fs').existsSync(outdir)) {
    get_description()
} else {
    setup()
}
