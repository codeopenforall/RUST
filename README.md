# Rust CVE Dataset for Transfer-Learning-Based Vulnerability Detection

Dataset used in **"Detecting Rust Code Vulnerabilities Through Transfer Learning"** (International Journal of Software Engineering and Knowledge Engineering, 2026), which introduces **TL-HGNN**, a transfer-learning framework that combines heterogeneous graph neural networks with LLM-based code translation to detect Rust vulnerabilities without Rust-specific training.

## Contents

| Directory | Samples | Description |
| --- | ---: | --- |
| `Data/Positive/` | 137 | Functions and snippets from real-world Rust CVEs, with the vulnerable code |
| `Data/Negative/` | 136 | Corresponding non-vulnerable (patched or benign) Rust code |

Files are named `<CVE-ID>_<CWE-ID>.rs` (or `.txt` where the CVE record supplies text rather than a standalone function); entries whose NVD record lists no CWE are marked `NVD-CWE`. The samples span memory-safety classes such as CWE-119, CWE-125, CWE-416, CWE-787 and CWE-908, as well as logic, injection and authorization classes (CWE-20, CWE-79, CWE-287, CWE-863).

## How it was used

The paper evaluates TL-HGNN on 82 real-world Rust CVE pairs across 44 CWEs drawn from this collection. Source-language models are trained on C and Java vulnerability data, represented as inter-procedural code property graphs (ICCPG), then transferred to Rust via LLM-translated code. TL-HGNN reaches an F1 of 66.24%, above the evaluated baselines.

## Usage

```bash
git clone https://github.com/codeopenforall/RUST
ls RUST/Data/Positive RUST/Data/Negative
```

## Related

- [RUST-VGA](https://github.com/codeopenforall/RUSTVGA) — LLM-generated, test-validated benchmark of 8,480 Rust programs (COMPSAC 2026)

## Responsible use

The positive samples contain real vulnerable code. Use them for research only; do not deploy.

## Citation

```bibtex
@article{khan2026detecting,
  title   = {Detecting Rust Code Vulnerabilities Through Transfer Learning},
  author  = {Khan, Irfan Ali and Luo, Yu and Xu, Weifeng and Xu, Dianxiang},
  journal = {International Journal of Software Engineering and Knowledge Engineering},
  year    = {2026}
}
```
