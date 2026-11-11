## Current XDoc Discrepancies & Improvement Candidates

1. **Outdated XDoc README**
   - The directory map still describes the previous architecture.
   - References directories and paths that have since moved or disappeared.
   - Rebuild the map to reflect the actual XDoc structure.

2. **XGPSE Documentation Structure**
   - XGPSE README calls itself an exact mirror but also says it does not mirror every source file.
   - Its listed source directories do not match the current `src` hierarchy.
   - Clarify its purpose, organizational relationship to GPSE, and documentation conventions.

3. **README and Index Inconsistencies**
   - Guides README describes several guides that do not currently exist.
   - Manifesto README describes a planned document structure that is not yet reflected in its directory.
   - Distinguish planned documents from missing documentation.

4. **Empty and Incomplete Documents**
   - Several files are empty or contain only minimal placeholder material.
   - Examples: `culture/diary.md`, `development/review/quality_assurance.md`, `sources/media.md`, `sources/repositories.md`, and several XGPSE mathematical documents.
   - Determine whether each should be developed, relocated, retained as a placeholder, or removed.

5. **Potentially Overlapping Records**
   - Review the responsibilities of `development/issues/audit.md`, `issues.md`, `status/notes.md`, `war_room/war_panel.md`, and `panel_001.md`.
   - Clarify the distinction between active issues, audit records, project status, historical war panels, and reusable templates.
   - Avoid multiple authoritative records for the same information.

6. **Naming and File Conventions**
   - Review inconsistent naming, capitalization, extensions, and terminology.
   - Example: `unit_cirlce.md` appears to contain a spelling error.
   - `XGPSE/systems/mission/core/mission_core` has no file extension.
   - Establish consistent conventions without renaming files unnecessarily.

7. **Chronicle Organization**
   - `culture/chronicles.md` is now a substantial historical record.
   - Consider an index, chapters, or separate historical entries to improve navigation while preserving the original material.

8. **Source Accuracy and Traceability**
   - Verify technical documentation against the actual source code.
   - Check internal links, references, attribution, and outdated paths.
   - Distinguish confirmed inaccuracies from items that simply have not been verified.

9. **Document Authority and Lifecycle**
   - Establish which documents are authoritative, supporting, historical, experimental, or proposed.
   - Make superseded decisions and historical material distinguishable from current project direction.

10. **Directory Responsibility**
    - Confirm that every top-level category has a clearly defined purpose.
    - Review whether information belongs in XDoc, XGPSE, GPSE source, or another repository location.
    - Avoid unnecessary nesting and premature creation of categories.

**Audit Note:** These are initial findings and review candidates, not authorization to relocate or delete material. Confirm each item's purpose and preservation requirements before making changes.

My additional observations
Three things deserve particular attention:
- The README problem is the clearest confirmed defect. It is still describing an older XDoc organization. That's our first cleanup target.
- XGPSE needs a precise definition. I like the idea of documenting GPSE by subsystem without duplicating source code. We should establish that distinction as a convention.
- The War Panel itself needs a defined role. We now have panel_001.md, war_panel.md, and THE_GREAT_XDOC_REFACTOR.md. They should have distinct responsibilities rather than becoming competing places to record the same decisions.
One caution: I have not yet performed a full link-by-link audit or reviewed every document's contents, so I haven't labeled unverified links or possible duplication as confirmed defects.

My proposed audit order: README and directory maps → XGPSE conventions → empty documents → document authority and overlap → full content review.