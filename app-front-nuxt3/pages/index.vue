<script setup lang="ts">
import init, { v1_add_int, v1_get_int, V1AddIntRequest, V1GetIntRequest } from "app-front-rust";

const diff = ref<number>(0);
const sum = ref<number>(0);
const err_msg = ref<string>("");

onBeforeMount(async () => {
  await init();
  await call_v1_get_int();
});

const call_v1_add_int = async () => {
  const req = V1AddIntRequest.new(diff.value);
  const res = await v1_add_int(req);

  if (res.is_ok()) {
    let res_ok = res.ok();
    if (res_ok) {
      sum.value = res_ok.sum;
      err_msg.value = "";
    }
  }
  if (res.is_err()) {
    let res_err = res.err();
    if (res_err) {
      err_msg.value = res_err.message;
    }
  }
};

const call_v1_get_int = async () => {
  const req = V1GetIntRequest.new();
  const res = await v1_get_int(req);

  if (res.is_ok()) {
    let res_ok = res.ok();
    if (res_ok) {
      sum.value = res_ok.sum;
      err_msg.value = "";
    }
  }
  if (res.is_err()) {
    let res_err = res.err();
    if (res_err) {
      console.error(res_err.error);
    }
  }
};

</script>

<template>
    <div>
        <h1>Rust and Nuxt3 App</h1>
        <input type="number" v-model="diff" />
        <button @click="call_v1_add_int">Add</button>
        <p>sum: {{ sum }}</p>
        <div v-if="err_msg">
            <p>Error: {{ err_msg }}</p>
        </div>
    </div>
</template>
