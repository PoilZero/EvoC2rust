# EvoC2rust Translation Report (vec_run_001)

## Run Summary
- Project name: vec_run_001
- Source repo: https://github.com/rxi/vec
- Artifacts folder: run_artifacts/vec_run_001
- LLM: Qwen (DashScope OpenAI-compatible), API key from BAILIAN_API_KEY
- Result: cargo build + cargo test succeeded (warnings only)

## Numbered Artifacts (in order)
1. 01_project/ - cloned C source
2. 02_llm_config.ini - LLM config snapshot
3. 03_c_metadata/ - C metadata output
4. 04_rust_metadata/ - Rust metadata output
5. 05_cache_gen/ - generation cache
6. 06_report_delim_fix.json - delimiter-fix report
7. 07_cache_delim_fix/ - delimiter-fix cache
8. 08_report_rule_fix.json - rule-fix report
9. 09_cache_rule_fix/ - rule-fix cache
10. 10_report_llm_repair.json - LLM repair report
11. 11_cache_llm_repair/ - LLM repair cache (pre-manual patch)
12. 12_cache_llm_repair_patched/ - LLM repair cache after manual patch
13. 13_final_project/ - final generated Rust project
14. 14_build_log.txt - cargo build log
15. 15_test_log.txt - cargo test log
16. 16_REPORT.md - this report

## Pipeline Execution
1. Metadata extraction
   - python scripts/extract_metadata.py --project_name vec_run_001
2. Generation (Qwen client)
   - cache dir: data/default/cache/evo-c2rust-v2-ds-gen-only-run001
3. Delimiter repair
   - cache dir: data/default/cache/evo-c2rust-v2-ds-delim-fix-run001
4. Rule-based repair
   - cache dir: data/default/cache/evo-c2rust-v2-ds-rule-fix-run001
5. LLM repair
   - cache dir: data/default/cache/evo-c2rust-v2-ds-llm-repair-run001
6. Final project generation
   - final_project/vec_run_001
7. Build and tests
   - cargo build
   - cargo test

## Fixes and Iterations
- LLM repair improved function compile rate but left one failing function.
- Build failed with E0282 on vec_insert_ due to ambiguous .cast() usage in c_memmove! call.
- Manual patch applied in cache (see 12_cache_llm_repair_patched) to use Ptr<u8> arithmetic and pass dst/src directly:
  - let mut src: Ptr<u8> = *data + idx * memsz;
  - let mut dst: Ptr<u8> = *data + (idx + 1) * memsz;
  - c_memmove!(dst, src, count * memsz);

## Verification Results
- Build: success (see 14_build_log.txt)
- Tests: success, 8 tests passed (see 15_test_log.txt)
- Warnings: numerous unused macro/function warnings from translation_utils and generated code

## Notes
- The directory 11_cache_llm_repair contains the raw LLM repair output.
- The directory 12_cache_llm_repair_patched contains the post-fix cache used to build and test.

# EvoC2rust Translation Report (vec_run_002)

## Run Summary
- Project name: vec_run_002
- Source repo: https://github.com/rxi/vec
- Artifacts folder: run_artifacts/vec_run_002
- LLM: Local Qwen3-4B-Thinking (vLLM, OpenAI-compatible)
- Result: cargo build + cargo test succeeded after client fix and manual patch

## Report
- Full report: run_artifacts/vec_run_002/26_REPORT.md
