<script>
import Schedule from "./components/Schedule.vue";
import ImportScheduleDialog from "./components/dialogs/ImportScheduleDialog.vue";
import Settings from "./components/Settings.vue";
import { Menu, ArrowLeft, ArrowRight, Refresh, Setting } from '@element-plus/icons-vue'
import { invoke } from "@tauri-apps/api/tauri";

export default {
  data() {
    return {
      weekId: 1,
      today: {},
      startOfSemester: new Date(2023, 1, 13),
      showMenu: false,
      scheduleName: "",
      defaultScheduleName: "",
      scheduleList: [],
      showImportScheduleDialog: false,
      showSettings: false
    }
  },
  components: {
    Schedule,
    Menu,
    ArrowLeft,
    ArrowRight,
    ImportScheduleDialog,
    Refresh,
    Setting,
    Settings
  },
  methods: {
    getToday() {
      var today = new Date();
      var weeks = Math.floor((today - this.startOfSemester) / (7 * 24 * 60 * 60 * 1000)) + 1;
      var day = today.getDay();
      return { week: weeks, day: day };
    },
    async updateScheduleList() {
      console.log("update schedule list");
      this.scheduleList = await invoke("get_schedule_list");
    },
    updateSettings(settings) {
      this.startOfSemester = new Date(settings["startOfSemester"]);
      this.defaultScheduleName = settings["defaultScheduleName"];
    },
    async loadSettings() {
      var settings = await invoke("get_or_default_settings");
      this.updateSettings(settings);
      this.scheduleName = this.defaultScheduleName;
    }
  },
  watch: {
    startOfSemester() {
      this.today = this.getToday();
      this.weekId = this.today.week;
    }
  },
  mounted() {
    this.today = this.getToday();
    this.weekId = this.today.week;
    this.updateScheduleList();
    this.loadSettings();
  }
}
</script>

<template>
  <el-row>
    <el-col :span="3" class="title-bar">
      <el-button class="title-bar no-border" @click="showMenu = true"><el-icon><Menu /></el-icon></el-button>
    </el-col>
    <el-col :span="6" class="title-bar">
      <div class="title-text">
        第{{ weekId }}周
      </div>
    </el-col>
    <el-col :span="11" class="title-bar">
      <div class="title-text">
        {{ scheduleName }}
      </div>
    </el-col>
    <el-col :span="4" class="title-bar flex-align-right">
      <el-button type="default" class="title-bar no-border" @click="weekId--;" v-if="weekId > 1"><el-icon><ArrowLeft /></el-icon></el-button>
      <el-button type="default" class="title-bar no-border" @click="weekId++;"><el-icon><ArrowRight /></el-icon></el-button>
    </el-col>
  </el-row>
  <el-row>
    <el-col :span="24">
      <Schedule :weekId="weekId" :scheduleName="scheduleName" :showWeekends="false" :today="today"/>
    </el-col>
  </el-row>

  <el-drawer v-model="showMenu" :with-header="false" direction="ltr" size="250px">
    <el-collapse class="no-divider-collapse">
      <el-collapse-item>
        <template #title>
          <div style="font-size: 1em !important;">
            切换课程表
          </div>
          <el-button class="no-border" @click.stop="updateScheduleList"><el-icon><Refresh /></el-icon></el-button>
        </template>
        <el-radio-group v-model="scheduleName" class="vertical-align no-border all-border-radius">
          <el-radio-button v-for="s in scheduleList" :label="s" >{{ s }}</el-radio-button>
        </el-radio-group>
        <br>
        <el-button class="no-border" @click="showImportScheduleDialog = true">导入课程表</el-button>
      </el-collapse-item>
      <el-row>
        <el-col :span="24">
          <el-button class="no-border settings" @click="showSettings = true"><el-icon class="el-icon--left"><Setting /></el-icon><el-text>设置</el-text></el-button>
        </el-col>
      </el-row>
    </el-collapse>
  </el-drawer>

  <ImportScheduleDialog v-model="showImportScheduleDialog" @schedule-imported="updateScheduleList"/>
  <Settings
    v-model:show-settings="showSettings"
    :schedule-list="scheduleList"
    @update="updateSettings"
  />
</template>

<style scoped>
.title-bar {
  min-height: 40px;
  max-height: 40px;
  background-color: white;
}

.title-text {
  font-size: 1.5em;
  padding-top: 10px;
}

.flex-align-right {
  display: flex;
  justify-content: flex-end;
}

.el-collapse-item__header {
  font-size: 1em !important;
  color: green;
}

.settings {
  font-size: 1em;
  /* color: black; */
  width: 100%;
  justify-content: start;
  padding-left: 0;
}
</style>