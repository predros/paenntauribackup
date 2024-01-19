<template>
  <v-btn-toggle
    v-model="store.current.sideBarType"
    class="vertical"
    color="primary"
    variant="text"
    rounded="0"
    mandatory
  >
    <v-btn
      width="40"
      height="45"
      :value="SideBarType.Select"
      @click="onChanged(SideBarType.Select)"
    >
      <v-icon size="28">mdi-cursor-default</v-icon>
      <v-tooltip activator="parent" location="end">
        {{ t("toolbars.side.select") }}
      </v-tooltip>
    </v-btn>

    <v-btn
      width="40"
      height="45"
      :value="SideBarType.NewNode"
      @click="onChanged(SideBarType.NewNode)"
    >
      <icon-base :width="32" :height="32" icon-name="node">
        <IconNode />
      </icon-base>
      <v-tooltip activator="parent" location="end">
        {{ t("toolbars.side.node") }}
      </v-tooltip>
    </v-btn>

    <v-btn
      width="40"
      height="45"
      :value="SideBarType.MatSec"
      @click="onChanged(SideBarType.MatSec)"
    >
      <icon-base :width="32" :height="32" icon-name="sections">
        <IconSections />
      </icon-base>
      <v-tooltip activator="parent" location="end">
        {{ t("toolbars.side.matSec") }}
      </v-tooltip>
    </v-btn>

    <v-btn
      width="40"
      height="45"
      :value="SideBarType.NewMember"
      @click="onChanged(SideBarType.NewMember)"
    >
      <icon-base :width="32" :height="32" icon-name="member">
        <IconMember />
      </icon-base>
      <v-tooltip activator="parent" location="end">
        {{ t("toolbars.side.member") }}
      </v-tooltip>
    </v-btn>

    <v-btn
      width="40"
      height="45"
      :value="SideBarType.Supports"
      @click="onChanged(SideBarType.Supports)"
    >
      <icon-base :width="32" :height="32" icon-name="supports">
        <IconSupports />
      </icon-base>
      <v-tooltip activator="parent" location="end">
        {{ t("toolbars.side.supports") }}
      </v-tooltip>
    </v-btn>

    <v-btn
      width="40"
      height="45"
      :value="SideBarType.Hinges"
      @click="onChanged(SideBarType.Hinges)"
    >
      <icon-base :width="32" :height="32" icon-name="hinge">
        <IconHinges />
      </icon-base>
      <v-tooltip activator="parent" location="end">
        {{ t("toolbars.side.hinges") }}
      </v-tooltip>
    </v-btn>

    <v-btn
      width="40"
      height="45"
      :value="SideBarType.NodalForces"
      @click="onChanged(SideBarType.NodalForces)"
    >
      <icon-base :width="32" :height="32" icon-name="force">
        <IconForce />
      </icon-base>
      <v-tooltip activator="parent" location="end">
        {{ t("toolbars.side.forces") }}
      </v-tooltip>
    </v-btn>

    <v-btn
      width="40"
      height="45"
      :value="SideBarType.MemberLoads"
      @click="onChanged(SideBarType.MemberLoads)"
    >
      <icon-base :width="32" :height="32" icon-name="loads">
        <IconLoads />
      </icon-base>
      <v-tooltip activator="parent" location="end">
        {{ t("toolbars.side.loads") }}
      </v-tooltip>
    </v-btn>

    <v-btn
      width="40"
      height="45"
      :value="SideBarType.ThermalLoads"
      @click="onChanged(SideBarType.ThermalLoads)"
    >
      <v-icon size="28">mdi-thermometer</v-icon>

      <v-tooltip activator="parent" location="end">
        {{ t("toolbars.side.temperature") }}
      </v-tooltip>
    </v-btn>
  </v-btn-toggle>
</template>

<script setup lang="ts">
import IconBase from "@/components/icons/IconBase.vue";
import IconForce from "@/components/icons/IconForce.vue";
import IconHinges from "@/components/icons/IconHinges.vue";
import IconLoads from "@/components/icons/IconLoads.vue";
import IconMember from "@/components/icons/IconMember.vue";
import IconNode from "@/components/icons/IconNode.vue";
import IconSections from "@/components/icons/IconSections.vue";
import IconSupports from "@/components/icons/IconSupports.vue";
import { SideBarType, ClickType } from "@/types/types";
import useGlobalStore from "@/state/global";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const store = useGlobalStore();

function onChanged(value: SideBarType): void {
  store.current.sideBarType = value;

  switch (value) {
    case SideBarType.NewNode:
      store.current.clickType = ClickType.NewNode;
      break;
    case SideBarType.NewMember:
      store.current.clickType = ClickType.NewMemberStart;
      break;
    default:
      store.current.clickType = ClickType.Select;
  }
}
</script>

<style scoped>
.vertical {
  display: flex;
  flex-flow: column;
  height: 100%;
  flex: 1;
}

.button {
  width: 60px;
  height: 60px;
}
</style>
