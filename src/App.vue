<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed, watch, reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { NSwitch } from "naive-ui";

enum TauriEvent {
  DragDrop = "tauri://drag-drop"
}

enum RustApi {
  StartConvert = "start_convert",
  StopConvert = "stop_convert",
}

enum FFmpegEvent {
  Error = "error",
  Finish = "finish",
  Progress = "progress",
}

const ffmpegPath = ref("/opt/homebrew/bin/ffmpeg");
const filePath = ref("");
const format = ref("ts");
const encode= ref("copy");
const isConverting = ref(false);
const logs = ref<string[]>([]);
const startTime = reactive({ hour: 0, minute: 0, second: 0 });
const endTime = reactive({ hour: 23, minute: 59, second: 59 });
const videoSize = reactive({ width: 1920, height: 1080 });
const videoSizeUsed = ref(false);
const unlistenFunctions = ref<UnlistenFn[]>([]);

const logBox = ref<HTMLElement | null>(null);

const watchLogs = computed(() => {
  return [...logs.value];
});

const formatOptions = [
  { value: "ts", label: ".ts" },
  { value: "mp4", label: ".mp4" },
  { value: "mkv", label: ".mkv" },
  { value: "mov", label: ".mov" },
];

const encodeOptions = [
  { value: "copy", label: "copy" },
  { value: "h264", label: "h264" },
  { value: "h265", label: "h265" },
];

/**
 * 註冊指令相關事件監聽 (用變數記錄下來)
 */
async function registerListener() {
  unlistenFunctions.value.push(await handleCommandError());
  unlistenFunctions.value.push(await handleCommandFinish());
  unlistenFunctions.value.push(await handleCommandProgress());
}

/**
 * 解除指令相關事件監聽 (執行記錄的unlisten())
 */
function unregisterListener() {
  unlistenFunctions.value.forEach((handle: () => void) => { handle(); });
}

/**
 * 監聽tauri的drag-drop功能
 */
function listenDragDrop() {

  listen(TauriEvent.DragDrop, (event: any) => {
    if (event?.payload?.paths.length > 0) {
      filePath.value = event.payload.paths[0];
    }
  });
}

/**
 * 切換按鍵功能 ()'結束' <=> '轉換')
 */
function handleConvert() {
  if (isConverting.value) { stop_convert(); return; }
  start_convert();
}

/**
 * 一直顯示兩位數字
 * @param val 
 */
function displayTime(value: number) {
  return value < 10 ? `0${value}` : `${value}`;
}

/**
 * 處理小時數在範圍之內 (max~min)
 * @param refVar 
 * @param min 
 * @param max 
 */
function onHourChange(refVar: any, min: number, max: number) {
  refVar.hour = Math.max(min, Math.min(max, refVar.hour));
}

/**
 * 處理分鐘數在範圍之內 (max~min)
 * @param refVar 
 * @param min 
 * @param max 
 */
function onMinuteChange(refVar: any, min: number, max: number) {
  refVar.minute = Math.max(min, Math.min(max, refVar.minute));
}

/**
 * 處理秒數在範圍之內 (max~min)
 * @param refVar 
 * @param min 
 * @param max 
 */
function onSecondChange(refVar: any, min: number, max: number) {
  refVar.second = Math.max(min, Math.min(max, refVar.second));
}

/**
 * 處理寬度在範圍之內 (max~min)
 * @param refVar 
 * @param min 
 * @param max 
 */
function onWidthChange(refVar: any, min: number) {
  refVar.width = Math.max(min, refVar.width);
}

/**
 * 處理高度在範圍之內 (max~min)
 * @param refVar 
 * @param min 
 * @param max 
 */
function onHeightChange(refVar: any, min: number) {
  refVar.height = Math.max(min, refVar.height);
}

/**
 * 啟動影片轉換功能
 */
async function start_convert() {

  const start_time = `${startTime.hour}:${startTime.minute}:${startTime.second}`;
  const end_time = `${endTime.hour}:${endTime.minute}:${endTime.second}`;
  const scale = videoSizeUsed.value ? `${videoSize.width}:${videoSize.height}` : ``

  console.log("start_convert", { scale: scale, videoSizeUsed: videoSizeUsed.value });

  logs.value = [];
  isConverting.value = true;

  await invoke(RustApi.StartConvert, {
    command: ffmpegPath.value,
    path: filePath.value,
    startTime: start_time,
    endTime: end_time,
    format: format.value,
    encode: encode.value,
    scale: scale
  });
}

/**
 * 停止影片轉換功能
 */
async function stop_convert() {
  await invoke(RustApi.StopConvert);
}

/**
 * 處理tauri傳來的finish訊息 (emit)
 */
async function handleCommandFinish() {

  return await listen<string>(FFmpegEvent.Finish, (event: any) => {
    const payload = event.payload as string;
    isConverting.value = false;
    logs.value.push(payload);
  });
}

/**
 * 處理tauri傳來的progress訊息 (emit)
 */
async function handleCommandProgress() {

  return await listen<string>(FFmpegEvent.Progress, (event: any) => {
    const payload = event.payload as string;
    logs.value.push(payload);
  });
}

/**
 * 處理tauri傳來的error訊息 (emit)
 */
async function handleCommandError() {

  return await listen<string>(FFmpegEvent.Error, (event: any) => {
    const payload = event.payload as string;
    isConverting.value = false;
    logs.value.push(payload);
  });
}
/**
 * 切換尺寸選項 (開啟/關閉)
 * @param value
 */
function onSwitch(isSwitch: boolean) {
  videoSizeUsed.value = isSwitch;
}

/**
 * 初始化變數值
 */
function initValue() {
  logs.value = [];
  isConverting.value = false;
}

onMounted(async () => {
  listenDragDrop();
  await registerListener();
  initValue();
});

onUnmounted(() => {
  unregisterListener();
});

watch(watchLogs, () => {
  setTimeout(() => { 
    if (logBox.value) { logBox.value.scrollTop = logBox.value.scrollHeight; }
  }, 0);
});
</script>

<template>
  <div class="main-drop-area">

    <div style="display: flex; width: 90%; align-items: center;">
      <input v-model="ffmpegPath" placeholder="ffmpeg路徑會顯示在這裡" style="flex: 1;" />
    </div>

    <div style="display: flex; width: 90%; align-items: center; margin-top: 12px;">
      <input v-model="filePath" placeholder="檔案路徑會顯示在這裡" style="flex: 1;" />
      <select v-model="format" class="select-option">
        <option v-for="option in formatOptions" :key="option.value" :value="option.value">{{ option.label }}</option>
      </select>
      <select v-model="encode" class="select-option">
        <option v-for="option in encodeOptions" :key="option.value" :value="option.value">{{ option.label }}</option>
      </select>
      <button type="button" style="margin-left: 8px;" :class="isConverting ? 'stopBtn' : 'startBtn'" @click="handleConvert">
        {{ isConverting ? '結束' : '轉換' }}
      </button>
    </div>

    <div style="display: flex; width: 90%; align-items: center; margin-top: 12px;">
      <span style="margin-right: 8px;">開始</span>
      <input type="number" min="0" max="23" v-model.number="startTime.hour" style="width: 60px;" placeholder="時" @input="onHourChange(startTime, 0, 23)" :value="displayTime(startTime.hour)" />
      <span style="margin: 0 8px;">:</span>
      <input type="number" min="0" max="59" v-model.number="startTime.minute" style="width: 60px;" placeholder="分" @input="onMinuteChange(startTime, 0, 59)" :value="displayTime(startTime.minute)" />
      <span style="margin: 0 8px;">:</span>
      <input type="number" min="0" max="59" v-model.number="startTime.second" style="width: 60px;" placeholder="秒" @input="onSecondChange(startTime, 0, 59)" :value="displayTime(startTime.second)" />
    </div>

    <div style="display: flex; width: 90%; align-items: center; margin-top: 12px;">
      <span style="margin-right: 8px;">結束</span>
      <input type="number" min="0" max="23" v-model.number="endTime.hour" style="width: 60px;" placeholder="時" @input="onHourChange(startTime, 0, 23)" :value="displayTime(endTime.hour)" />
      <span style="margin: 0 8px;">:</span>
      <input type="number" min="0" max="59" v-model.number="endTime.minute" style="width: 60px;" placeholder="分" @input="onMinuteChange(startTime, 0, 59)" :value="displayTime(endTime.minute)" />
      <span style="margin: 0 8px;">:</span>
      <input type="number" min="0" max="59" v-model.number="endTime.second" style="width: 60px;" placeholder="秒" @input="onSecondChange(startTime, 0, 59)" :value="displayTime(endTime.second)" />
    </div>

    <div style="display: flex; width: 90%; align-items: center; margin-top: 12px;">
      <span style="margin-right: 8px;">尺寸</span>
      <input type="number" min="-1" v-model.number="videoSize.width" style="width: 60px;" placeholder="寬" @input="onWidthChange(videoSize, -1)" :value="videoSize.width" />
      <span style="margin: 0 8px;">:</span>
      <input type="number" min="-1" v-model.number="videoSize.height" style="width: 60px;" placeholder="高" @input="onHeightChange(videoSize, -1)" :value="videoSize.height" />
      <n-switch @update:value="onSwitch" style="margin-left: 16px;">
        <template #checked>ON</template>
        <template #unchecked>OFF</template>
      </n-switch>
    </div>

    <div ref="logBox" class="log-box" style="margin-top: 12px;">
      <div v-for="(line, idx) in logs" :key="idx" class="log-segment">{{ line }}</div>
    </div>

  </div>
</template>

<style scoped>
.main-drop-area {
  position: absolute;
  top: 10px;
  left: 10px;
  right: 10px;
  bottom: 10px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border: 2px dashed #888;
  background: transparent;
}

.startBtn {
  background: #43a047 !important;
  color: #fff !important;
  border: 1px solid #1b5e20 !important;
}

.stopBtn {
  background: #e53935 !important;
  color: #fff !important;
  border: 1px solid #b71c1c !important;
}

.log-segment {
  border-bottom: 1px solid #444;
  padding: 4px 0 8px 0;
  margin-bottom: 4px;
  word-break: break-all;
}

.log-segment:last-child {
  border-bottom: none;
}

.log-box {
  width: 90%;
  background: #222;
  color: #fff;
  font-size: 14px;
  border-radius: 6px;
  padding: 8px;
  height: 200px;
  max-height: 200px;
  overflow: auto;
}

.select-option {
  margin-left: 8px;
  padding: 0.6em 1.2em;
  border-radius: 8px;
  border: 1px solid #bbb; 
  background: #0f0f0f;
  color: #f0f0f0;
  font-size: 1em; 
  font-family: inherit; 
  cursor: pointer; 
  height: 44px; 
  line-height: 44px
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>