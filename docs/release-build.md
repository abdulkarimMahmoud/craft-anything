# بناء حزم الإصدار محليًا (لينكس)

هذه الخطوات تبني حزم لينكس (`‎.deb` و‎`.rpm` و‎`.tar.gz`) على جهازك من فرع العربيّة RTL.
البناء يحتاج ذاكرة كافية (4GB على الأقل للحزمة الكاملة) ومساحة قرص ~8GB.

## 1. المتطلبات

- Rust مستقر: `rustup toolchain install stable`
- حزم النظام (دبيان/أوبونتو):

  ```sh
  sudo apt-get update
  sudo apt-get install -y git libxkbcommon-dev libwayland-dev libx11-dev \
    libxrandr-dev libxi-dev libgl1-mesa-dev libgtk-3-dev
  ```

- `nfpm` لحزمتي deb/rpm (https://nfpm.goreleaser.com/install/) — أو:

  ```sh
  curl -sL https://github.com/goreleaser/nfpm/releases/download/v2.48.0/nfpm_2.48.0_Linux_x86_64.tar.gz \
    | sudo tar -xz -C /usr/local/bin nfpm
  ```

## 2. الاستنساخ والخطوط

```sh
git clone https://github.com/USER/craft-anything.git
cd craft-anything
git checkout feature/ar-rtl   # أو وسم الإصدار، مثل: git checkout v0.4.0-ar2
git clone https://github.com/storytold/craft-fonts.git ../craft-fonts
export CRAFT_FONTS_DIR="$PWD/../craft-fonts"   # ضروري: يضمّن Noto Sans Arabic
export DIST="$PWD/../packages"                 # أين تُكتب الحزم (اختياري)
```

## 3. البناء والتحزيم

```sh
bash packaging/linux/package.sh --formats "deb rpm tar"
```

## 4. النواتج والتثبيت

في `$DIST` (افتراضيًا `dist/release`):

- `craft-anything-0.4.0-linux-x86_64.deb` — التثبيت: `sudo apt install ./craft-anything-*.deb`
- `craft-anything-0.4.0-linux-x86_64.rpm` — التثبيت: `sudo dnf install ./craft-anything-*.rpm`
- `craft-anything-0.4.0-linux-x86_64.tar.gz` — شجرة جاهزة: `tar -xzf craft-anything-*.tar.gz`

يفحص السكربت الحزم تلقائيًا (`‎--version` لكل ثنائية).

## ملاحظات

- ابنِ على توزيعة قديمة نسبيًا (Ubuntu 22.04، glibc 2.35) لتعمل الثنائيات على الأحدث أيضًا.
- حزم Windows (الـ MSI وحزمة NSIS والمحمولة) تُبنى على GitHub Actions من
  `.github/workflows/windows-release.yml` — لا تُبنَ محليًا على لينكس.
- رقم الإصدار من `[workspace.package] version` في `Cargo.toml` — غيّره هناك فقط.
