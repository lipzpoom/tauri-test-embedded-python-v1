# Tauri Test Embedded Python v1

โปรเจกต์ Tauri แอพพลิเคชั่น Desktop ที่รวม Vue.js frontend กับ Python backend แบบฝังตัว (Embedded Python) สำหรับการจัดการข้อมูล Trading History

## คุณสมบัติ

- 🚀 **Tauri Desktop App** - แอพพลิเคชั่น desktop ที่เร็วและปลอดภัย
- 🎨 **Vue.js Frontend** - UI ที่ทันสมัยและตอบสนองได้ดี
- 🐍 **Embedded Python** - Python server แบบฝังตัวสำหรับประมวลผลข้อมูล
- 📊 **Trading History API** - การจัดการข้อมูลการซื้อขายแบบเรียลไทม์
- 📦 **Portable Build** - สร้างไฟล์ติดตั้งแบบ portable

## ความต้องการของระบบ

- **Node.js** >= 18.0.0
- **npm** หรือ **yarn**
- **Rust** >= 1.70.0
- **PowerShell** (สำหรับ Windows)
- **Python** 3.11+ (จะถูกดาวน์โหลดอัตโนมัติ)

## การติดตั้ง

### 1. Clone Repository

```bash
git clone https://github.com/lipzpoom/tauri-test-embedded-python-v1.git
cd tauri-test-embedded-python-v1
```

### 2. ติดตั้ง Dependencies

```bash
npm install
```

### 3. ติดตั้ง Tauri CLI

```bash
npm install -g @tauri-apps/cli
```

### 4. ติดตั้ง Rust (หากยังไม่มี)

```bash
# Windows
winget install Rustlang.Rustup

# หรือดาวน์โหลดจาก https://rustup.rs/
```

## การใช้งาน

### Development Mode

#### วิธีที่ 1: รัน Frontend และ Backend แยกกัน

```bash
# Terminal 1: รัน Vue.js development server
npm run dev

# Terminal 2: รัน Tauri development
npm run tauri:dev
```

#### วิธีที่ 2: รัน Development พร้อม Python Setup

```bash
# รัน development mode พร้อมการตั้งค่า Python อัตโนมัติ
npm run dev:with-python
```

### Production Build

#### สร้าง Portable Application

```bash
# สร้าง portable build (แนะนำ)
npm run build:bundle
```

#### สร้าง Standard Installer

```bash
# สร้าง standard installer
npm run build
npm run tauri build
```

## คำสั่งที่สำคัญ

| คำสั่ง                    | คำอธิบาย                           |
| ------------------------- | ---------------------------------- |
| `npm run dev`             | รัน Vue.js development server      |
| `npm run tauri:dev`       | รัน Tauri development mode         |
| `npm run dev:with-python` | รัน development พร้อม Python setup |
| `npm run setup-python`    | ติดตั้ง Python แบบ portable        |
| `npm run build:bundle`    | สร้าง portable application         |
| `npm run tauri build`     | สร้าง production build             |

## โครงสร้างโปรเจกต์

```
├── src/                    # Vue.js Frontend
│   ├── App.vue            # Main Vue component
│   ├── main.js            # Vue app entry point
│   └── assets/            # Static assets
├── src-tauri/             # Tauri Backend
│   ├── src/               # Rust source code
│   │   ├── main.rs        # Main Rust entry point
│   │   └── lib.rs         # Library functions
│   ├── python/            # Python Backend
│   │   ├── main.py        # Python HTTP server
│   │   └── python-dist/   # Python runtime (auto-generated)
│   ├── Cargo.toml         # Rust dependencies
│   ├── tauri.conf.json    # Tauri configuration
│   └── setup-python.*     # Python setup scripts
├── public/                # Public static files
├── package.json           # Node.js dependencies and scripts
└── vite.config.js         # Vite configuration
```

## การใช้งาน API

### Trading History Endpoint

```javascript
// ใน Vue component
async function fetchTradingHistory() {
  try {
    const result = await invoke("get_trading_history");
    console.log(result);
  } catch (error) {
    console.error("Error:", error);
  }
}
```

### Python API Endpoints

- `GET /health` - ตรวจสอบสถานะของ Python server
- `GET /trading-history` - ดึงข้อมูล trading history

## การแก้ไขปัญหา

### ปัญหา Python ไม่ทำงาน

```bash
# ลบ python-dist และติดตั้งใหม่
Remove-Item -Recurse -Force src-tauri/python/python-dist
npm run setup-python
```

### ปัญหา Build ไม่สำเร็จ

```bash
# ล้าง Tauri cache
cargo clean
npm run build:bundle
```

### ปัญหา Port ถูกใช้งาน

ตรวจสอบไฟล์ `src-tauri/python/main.py` และเปลี่ยน port ในบรรทัด:

```python
PORT = 8001  # เปลี่ยนเป็น port อื่น
```

## การ Deploy

### Windows Portable

1. รัน `npm run build:bundle`
2. ไฟล์ portable จะอยู่ในโฟลเดอร์ `src-tauri/target/release/`
3. แจกจ่ายไฟล์ `.exe` ได้เลย (ไม่ต้องติดตั้ง)

### Windows Installer

1. รัน `npm run tauri build`
2. ไฟล์ installer จะอยู่ในโฟลเดอร์ `src-tauri/target/release/bundle/`

## การพัฒนาต่อ

### เพิ่ม Python Packages

1. แก้ไขไฟล์ `setup-python.ps1`
2. เพิ่ม package ในส่วน pip install
3. รัน `npm run setup-python` อีกครั้ง

### เพิ่ม Tauri Commands

1. เพิ่ม function ในไฟล์ `src-tauri/src/lib.rs`
2. เพิ่ม command ในไฟล์ `src-tauri/src/main.rs`
3. เรียกใช้ใน Vue component ด้วย `invoke("command_name")`

## การสนับสนุน

หากพบปัญหาหรือต้องการความช่วยเหลือ:

1. เปิด [Issue](https://github.com/lipzpoom/tauri-test-embedded-python-v1/issues) ในโปรเจกต์
2. อ่าน [Tauri Documentation](https://tauri.app/)
3. อ่าน [Vue.js Documentation](https://vuejs.org/)

---

**หมายเหตุ:** โปรเจกต์นี้ยังอยู่ในช่วงการพัฒนา (v0.1.0) และอาจมีการเปลี่ยนแปลงในอนาคต
