# Recon Workflow

أداة أتمتة لعمليات البحث (Reconnaissance) واكتشاف الثغرات مبنية بلغة Rust. تقوم الأداة بربط `subfinder`، `httpx`، و `nuclei` في سير عمل تسلسلي لتسريع عملية الاستطلاع.

## المتطلبات الأساسية

يجب توفر الأدوات التالية وتكوينها ضمن متغير مسار النظام (`$PATH`):
* `subfinder`
* `httpx`
* `nuclei`
* بيئة عمل Rust (`cargo`).

## البناء والتثبيت

لتوليد نسخة تنفيذية محسنة الأداء (Release):
```bash
cargo build --release
