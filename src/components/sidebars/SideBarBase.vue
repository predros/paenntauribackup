<template>
  <v-card
    flat
    class="pt-10 pl-4 pr-6"
  >
    <SideBarNode
      :class="store.current.sideBarType == SideBarType.NewNode ? '' : 'd-none'"
    />
    <SideBarMatSec
      :class="store.current.sideBarType == SideBarType.MatSec ? '' : 'd-none'"
    />
    <SideBarMember
      :class="
        store.current.sideBarType == SideBarType.NewMember ? '' : 'd-none'
      "
    />
    <SideBarSupports
      :class="store.current.sideBarType == SideBarType.Supports ? '' : 'd-none'"
    />
    <SideBarHinges
      :class="store.current.sideBarType == SideBarType.Hinges ? '' : 'd-none'"
    />
    <SideBarForces
      :class="
        store.current.sideBarType == SideBarType.NodalForces ? '' : 'd-none'
      "
    />
    <SideBarLoads
      :class="
        store.current.sideBarType == SideBarType.MemberLoads ? '' : 'd-none'
      "
    />
    <SideBarTemperature
      :class="
        store.current.sideBarType == SideBarType.ThermalLoads ? '' : 'd-none'
      "
    />
    <SideBarResult
      :class="store.current.sideBarType == SideBarType.Result ? '' : 'd-none'"
    />
  </v-card>
</template>

<script setup lang="ts">
import { SideBarType } from "@/types/types";
import useGlobalStore from "@/state/global";

import SideBarNode from "./SideBarNode.vue";
import SideBarMatSec from "./SideBarMatSec.vue";
import SideBarMember from "./SideBarMember.vue";
import SideBarSupports from "./SideBarSupports.vue";
import SideBarHinges from "./SideBarHinges.vue";
import SideBarForces from "./SideBarForces.vue";
import SideBarLoads from "./SideBarLoads.vue";
import SideBarTemperature from "./SideBarTemperature.vue";
import SideBarResult from "./SideBarResult.vue";

const store = useGlobalStore();
</script>
