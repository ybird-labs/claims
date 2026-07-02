#!/usr/bin/env python3
"""Generate spike/fixtures_carbon/ from a ybird-labs/carbon-project checkout.

Usage:
    python3 tools/gen_carbon_fixtures.py /path/to/carbon-project

Reads (read-only):
    progress/site_index.json
    progress/sites/<farm_key>.json   for the SITES below

Writes deterministic JSON (sorted keys, 2-space indent, trailing newline) so
committed fixtures are byte-reproducible from the same checkout. No
timestamps, clocks, or absolute paths appear in the output.

Anonymization: farmer/farm/holder names and plot names are never copied.
The landholder is replaced by a pseudonym derived from stable keys
("Landholder <ISO>-<farmer_id>"). Keys, dates, land-use values, acreages,
plot registration IDs, counts, outcomes, and versions are kept real. The
script self-checks that no name observed in site_index.json leaks into any
emitted file.

Airtable record IDs (retrieval coordinates) are recorded in manifest.json
only — per the domain-model resolution that retrieval coordinates belong to
the witnessed submission/audit layer, never to claim content.
"""

import json
import re
import sys
from pathlib import Path

# The stress-test slice: one CZ site (SL-009 not_satisfied) and one SK site
# (SL-009 satisfied), so the four requirements below span all three
# tri-state outcomes.
SITES = ["01.0035.00035", "02.4368.00441"]
REQUIREMENTS = [
    "C06-REGISTRATION-SL-003",
    "C06-REGISTRATION-SL-006",
    "C06-REGISTRATION-SL-007",
    "C06-REGISTRATION-SL-009",
]

NS = "https://carboneg.example/"
XSD = "http://www.w3.org/2001/XMLSchema#"
SOURCE_EVIDENCE_SCHEMA = NS + "schema/source-evidence/1.0.0"
DERIVATION_SCHEMA = NS + "schema/derivation/1.0.0"
JUDGMENT_SCHEMA = NS + "schema/requirement-judgment/1.0.0"
START_DATE_RULE = NS + "rule/earliest-active-soil-sampling-date/1.0.0"

SL007_NOTES_RE = re.compile(
    r"has (\d+) active Soil Sampling record\(s\) in Airtable, and the "
    r"earliest parsed sampling date is '(\d{4}-\d{2}-\d{2})'"
)


def site_iri(farm_key):
    return f"{NS}site/{farm_key}"


def plot_iri(plot_key):
    return f"{NS}plot/{plot_key}"


def requirement_iri(requirement_id):
    return f"{NS}requirement/{requirement_id}"


def source_evidence_context():
    se = SOURCE_EVIDENCE_SCHEMA + "/"
    return {
        "xsd": XSD,
        "se": se,
        "SiteRecord": "se:SiteRecord",
        "PlotRecord": "se:PlotRecord",
        "SamplingSummary": "se:SamplingSummary",
        "id": "@id",
        "type": "@type",
        "site": {"@id": "se:site", "@type": "@id"},
        "plot": {"@id": "se:plot", "@type": "@id"},
        "part_of_site": {"@id": "se:part_of_site", "@type": "@id"},
        "farm_key": "se:farm_key",
        "plot_key": "se:plot_key",
        "country_iso": "se:country_iso",
        "land_use": "se:land_use",
        "active_acreage": {"@id": "se:active_acreage", "@type": "xsd:double"},
        "acreage_hectares": {"@id": "se:acreage_hectares", "@type": "xsd:double"},
        "plot_count": {"@id": "se:plot_count", "@type": "xsd:integer"},
        "plots": {"@id": "se:plots", "@type": "@id"},
        "holder": "se:holder",
        "registration_id": "se:registration_id",
        "record_count": {"@id": "se:record_count", "@type": "xsd:integer"},
        "earliest_sampling_date": {
            "@id": "se:earliest_sampling_date",
            "@type": "xsd:date",
        },
    }


def source_evidence_schema():
    return {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": SOURCE_EVIDENCE_SCHEMA + "/schema.json",
        "title": "SourceEvidence",
        "description": (
            "Source-system facts about carbon-project entities. Entity "
            "identifiers (farm_key, plot_key) are claim content; retrieval "
            "coordinates (which Airtable row / workbook produced the fact) "
            "belong to the witnessed submission record, not here. "
            "Hand-authored as-if generated from a LinkML claim-type schema."
        ),
        "oneOf": [
            {
                "type": "object",
                "required": [
                    "type",
                    "site",
                    "farm_key",
                    "country_iso",
                    "land_use",
                    "active_acreage",
                    "plot_count",
                    "plots",
                    "holder",
                ],
                "properties": {
                    "type": {"const": "SiteRecord"},
                    "site": {"type": "string", "format": "uri"},
                    "farm_key": {"type": "string", "pattern": "^[0-9.]+$"},
                    "country_iso": {"type": "string", "minLength": 2, "maxLength": 2},
                    "land_use": {"type": "string", "minLength": 1},
                    "active_acreage": {"type": "number", "exclusiveMinimum": 0},
                    "plot_count": {"type": "integer", "minimum": 1},
                    "plots": {
                        "type": "array",
                        "minItems": 1,
                        "items": {"type": "string", "format": "uri"},
                    },
                    "holder": {"type": "string", "minLength": 1},
                },
                "additionalProperties": False,
            },
            {
                "type": "object",
                "required": [
                    "type",
                    "plot",
                    "plot_key",
                    "part_of_site",
                    "land_use",
                    "acreage_hectares",
                    "registration_id",
                ],
                "properties": {
                    "type": {"const": "PlotRecord"},
                    "plot": {"type": "string", "format": "uri"},
                    "plot_key": {"type": "string", "pattern": "^[0-9.]+$"},
                    "part_of_site": {"type": "string", "format": "uri"},
                    "land_use": {"type": "string", "minLength": 1},
                    "acreage_hectares": {"type": "number", "exclusiveMinimum": 0},
                    "registration_id": {"type": "string", "minLength": 1},
                },
                "additionalProperties": False,
            },
            {
                "type": "object",
                "required": ["type", "site", "record_count", "earliest_sampling_date"],
                "properties": {
                    "type": {"const": "SamplingSummary"},
                    "site": {"type": "string", "format": "uri"},
                    "record_count": {"type": "integer", "minimum": 1},
                    "earliest_sampling_date": {
                        "type": "string",
                        "pattern": "^[0-9]{4}-[0-9]{2}-[0-9]{2}$",
                    },
                },
                "additionalProperties": False,
            },
        ],
    }


def derivation_context():
    dv = DERIVATION_SCHEMA + "/"
    return {
        "xsd": XSD,
        "dv": dv,
        "Derivation": "dv:Derivation",
        "id": "@id",
        "type": "@type",
        "subject": {"@id": "dv:subject", "@type": "@id"},
        "property": "dv:property",
        "value": {"@id": "dv:value", "@type": "xsd:date"},
        "rule": {"@id": "dv:rule", "@type": "@id"},
        "derived_from": {"@id": "dv:derived_from", "@type": "@id"},
    }


def derivation_schema():
    return {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": DERIVATION_SCHEMA + "/schema.json",
        "title": "Derivation",
        "description": (
            "A value derived from other claims by a named rule, because the "
            "source systems expose no authoritative field. derived_from cites "
            "the input ClaimIRIs; the derivation is structured content, not "
            "prose notes. Hand-authored as-if generated from LinkML."
        ),
        "type": "object",
        "required": ["type", "subject", "property", "value", "rule", "derived_from"],
        "properties": {
            "type": {"const": "Derivation"},
            "subject": {"type": "string", "format": "uri"},
            "property": {"type": "string", "minLength": 1},
            "value": {"type": "string", "pattern": "^[0-9]{4}-[0-9]{2}-[0-9]{2}$"},
            "rule": {"type": "string", "format": "uri"},
            "derived_from": {
                "type": "array",
                "minItems": 1,
                "items": {"type": "string", "format": "uri"},
            },
        },
        "additionalProperties": False,
    }


def judgment_context():
    rj = JUDGMENT_SCHEMA + "/"
    return {
        "xsd": XSD,
        "rj": rj,
        "RequirementJudgment": "rj:RequirementJudgment",
        "id": "@id",
        "type": "@type",
        "requirement": {"@id": "rj:requirement", "@type": "@id"},
        "requirement_id": "rj:requirement_id",
        "credit_class_version": "rj:credit_class_version",
        "site": {"@id": "rj:site", "@type": "@id"},
        "outcome": "rj:outcome",
        "evidence": {"@id": "rj:evidence", "@type": "@id"},
        "judge": {"@id": "rj:judge", "@type": "@id"},
        "judged_at": {"@id": "rj:judged_at", "@type": "xsd:dateTime"},
    }


def judgment_schema():
    return {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": JUDGMENT_SCHEMA + "/schema.json",
        "title": "RequirementJudgment",
        "description": (
            "A user-space judgment that a site satisfies (or not) a numbered "
            "registration requirement under a specific credit-class version. "
            "Tri-state outcome and multi-claim evidence are user-space "
            "vocabulary: the engine's own validation-result schema stays "
            "binary and single-target. The normative rule version "
            "(credit_class_version) is content, not an engine schema version. "
            "Hand-authored as-if generated from LinkML."
        ),
        "type": "object",
        "required": [
            "type",
            "requirement",
            "requirement_id",
            "credit_class_version",
            "site",
            "outcome",
            "evidence",
            "judge",
            "judged_at",
        ],
        "properties": {
            "type": {"const": "RequirementJudgment"},
            "requirement": {"type": "string", "format": "uri"},
            "requirement_id": {"type": "string", "pattern": "^C06-[A-Z]+-[SM]L-[0-9]{3}$"},
            "credit_class_version": {"type": "string", "minLength": 1},
            "site": {"type": "string", "format": "uri"},
            "outcome": {"enum": ["satisfied", "not_satisfied", "unclear"]},
            "evidence": {
                "type": "array",
                "minItems": 2,
                "items": {"type": "string", "format": "uri"},
            },
            "judge": {"type": "string", "format": "uri"},
            "judged_at": {"type": "string", "format": "date-time"},
        },
        "additionalProperties": False,
    }


def dump(path, obj):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(obj, indent=2, sort_keys=True, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def build_site_fixtures(out_dir, index_entry, progress):
    farm_key = index_entry["farm_key"]
    holder = f"Landholder {index_entry['country_iso']}-{index_entry['farmer_id']}"
    site_dir = out_dir / "sites" / farm_key
    se_ctx = source_evidence_context()

    evidence_files = {}

    site_record = {
        "@context": se_ctx,
        "type": "SiteRecord",
        "site": site_iri(farm_key),
        "farm_key": farm_key,
        "country_iso": index_entry["country_iso"],
        "land_use": index_entry["land_use"],
        "active_acreage": index_entry["active_acreage"],
        "plot_count": index_entry["plot_count"],
        "plots": [plot_iri(k) for k in index_entry["plot_keys"]],
        "holder": holder,
    }
    dump(site_dir / "site_record.jsonld", site_record)
    evidence_files["site_record"] = "site_record.jsonld"

    plot_files = []
    for plot in index_entry["plots"]:
        record = {
            "@context": se_ctx,
            "type": "PlotRecord",
            "plot": plot_iri(plot["plot_key"]),
            "plot_key": plot["plot_key"],
            "part_of_site": site_iri(farm_key),
            "land_use": plot["land_use"],
            "acreage_hectares": plot["acreage_hectares"],
            "registration_id": plot["plot_registration_id"],
        }
        name = f"plot_{plot['plot_id']}.jsonld"
        dump(site_dir / name, record)
        plot_files.append(name)
    evidence_files["plots"] = plot_files

    requirements = {r["requirement_id"]: r for r in progress["requirements"]}
    sl007 = requirements["C06-REGISTRATION-SL-007"]
    match = SL007_NOTES_RE.search(sl007["notes"])
    if not match:
        raise SystemExit(
            f"{farm_key}: could not extract sampling summary from SL-007 notes"
        )
    record_count, earliest_date = int(match.group(1)), match.group(2)

    sampling = {
        "@context": se_ctx,
        "type": "SamplingSummary",
        "site": site_iri(farm_key),
        "record_count": record_count,
        "earliest_sampling_date": earliest_date,
    }
    dump(site_dir / "sampling_summary.jsonld", sampling)
    evidence_files["sampling"] = "sampling_summary.jsonld"

    # Build plan for runtime claim construction. Evidence entries name fixture
    # files (resolved to ClaimIRIs after admission) or "derivation" (resolved
    # to the derivation ClaimIRI). Outcomes come from the real progress file.
    evidence_by_requirement = {
        "C06-REGISTRATION-SL-003": ["site_record.jsonld"] + plot_files,
        "C06-REGISTRATION-SL-006": ["site_record.jsonld"] + plot_files,
        "C06-REGISTRATION-SL-007": ["derivation", "sampling_summary.jsonld"],
        "C06-REGISTRATION-SL-009": ["derivation"] + plot_files,
    }
    plan = {
        "farm_key": farm_key,
        "site": site_iri(farm_key),
        "credit_class_version": progress["credit_class_version"],
        "judged_at": progress["generated_at"],
        "derivation": {
            "subject": site_iri(farm_key),
            "property": "site_project_start_date",
            "value": earliest_date,
            "rule": START_DATE_RULE,
            "inputs": ["sampling_summary.jsonld", "site_record.jsonld"],
        },
        "judgments": [
            {
                "requirement_id": rid,
                "requirement": requirement_iri(rid),
                "outcome": requirements[rid]["outcome"],
                "evidence": evidence_by_requirement[rid],
            }
            for rid in REQUIREMENTS
        ],
    }
    dump(site_dir / "judgments.json", plan)

    return {
        "farm_key": farm_key,
        "airtable_record_ids": {
            "farmer": index_entry["source"]["farmer"]["airtable_record_id"],
            "farm": index_entry["source"]["farm"]["airtable_record_id"],
        },
    }


def forbidden_names(index_entries):
    """Names that must not appear in any emitted fixture. Names shorter than
    three characters (e.g. single-letter plot labels) are not identifying and
    would false-positive on ordinary JSON."""
    names = set()
    for entry in index_entries:
        for field in ("farmer_name", "farm_name", "project_area_name"):
            value = entry.get(field)
            if value:
                names.add(value)
        for plot in entry.get("plots", []):
            if plot.get("plot_name"):
                names.add(plot["plot_name"])
    return {name for name in names if len(name) >= 3}


def main():
    if len(sys.argv) != 2:
        raise SystemExit(__doc__)
    source = Path(sys.argv[1])
    out_dir = Path(__file__).resolve().parent.parent / "fixtures_carbon"

    site_index = json.loads(
        (source / "progress" / "site_index.json").read_text(encoding="utf-8")
    )
    by_key = {entry["farm_key"]: entry for entry in site_index["sites"]}

    dump(out_dir / "source-evidence.context.jsonld", {"@context": source_evidence_context()})
    dump(out_dir / "source-evidence.schema.json", source_evidence_schema())
    dump(out_dir / "derivation.context.jsonld", {"@context": derivation_context()})
    dump(out_dir / "derivation.schema.json", derivation_schema())
    dump(out_dir / "requirement-judgment.context.jsonld", {"@context": judgment_context()})
    dump(out_dir / "requirement-judgment.schema.json", judgment_schema())

    manifest_sites = []
    for farm_key in SITES:
        progress = json.loads(
            (source / "progress" / "sites" / f"{farm_key}.json").read_text(
                encoding="utf-8"
            )
        )
        manifest_sites.append(
            build_site_fixtures(out_dir, by_key[farm_key], progress)
        )

    dump(
        out_dir / "manifest.json",
        {
            "generator": "tools/gen_carbon_fixtures.py",
            "source_repository": "ybird-labs/carbon-project",
            "source_files": [
                "progress/site_index.json",
                *(f"progress/sites/{k}.json" for k in SITES),
            ],
            "requirements": REQUIREMENTS,
            "pseudonym_rule": "Landholder <country_iso>-<farmer_id>; names never copied",
            "retrieval_coordinates_note": (
                "Airtable record IDs below are retrieval coordinates. They are "
                "recorded here (audit-side documentation) and deliberately kept "
                "out of claim content, which carries entity identifiers only."
            ),
            "sites": manifest_sites,
        },
    )

    # Self-check: no real name from the site index may appear in any output.
    banned = forbidden_names(site_index["sites"])
    for path in sorted(out_dir.rglob("*")):
        if not path.is_file():
            continue
        text = path.read_text(encoding="utf-8")
        for name in banned:
            if name in text:
                raise SystemExit(f"name leak: {name!r} in {path}")

    print(f"wrote {sum(1 for p in out_dir.rglob('*') if p.is_file())} files to {out_dir}")


if __name__ == "__main__":
    main()
