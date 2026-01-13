<script setup lang="ts">
import { onMounted, ref, onUnmounted } from "vue";
import * as THREE from "three";

const container = ref<HTMLElement | null>(null);
let scene: THREE.Scene;
let camera: THREE.PerspectiveCamera;
let renderer: THREE.WebGLRenderer;
let car: THREE.Group;

onMounted(() => {
  if (!container.value) return;

  // Scene setup
  scene = new THREE.Scene();
  camera = new THREE.PerspectiveCamera(
    75,
    container.value.clientWidth / container.value.clientHeight,
    0.1,
    1000
  );
  renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
  renderer.setSize(container.value.clientWidth, container.value.clientHeight);
  container.value.appendChild(renderer.domElement);

  // Lighting
  const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
  scene.add(ambientLight);
  const directionalLight = new THREE.DirectionalLight(0x38bdf8, 1);
  directionalLight.position.set(5, 5, 5);
  scene.add(directionalLight);

  // Simple Car Geometry (Boxy for mock)
  car = new THREE.Group();

  // Body
  const bodyGeo = new THREE.BoxGeometry(2, 0.5, 4);
  const bodyMat = new THREE.MeshPhongMaterial({ color: 0x1e293b });
  const body = new THREE.Mesh(bodyGeo, bodyMat);
  car.add(body);

  // Cabin
  const cabinGeo = new THREE.BoxGeometry(1.5, 0.6, 2);
  const cabinMat = new THREE.MeshPhongMaterial({
    color: 0x334155,
    transparent: true,
    opacity: 0.7,
  });
  const cabin = new THREE.Mesh(cabinGeo, cabinMat);
  cabin.position.y = 0.5;
  car.add(cabin);

  // Wheels
  const wheelGeo = new THREE.CylinderGeometry(0.3, 0.3, 0.2, 32);
  const wheelMat = new THREE.MeshPhongMaterial({ color: 0x000000 });
  const wheelPositions = [
    [-1.1, -0.25, 1.5],
    [1.1, -0.25, 1.5],
    [-1.1, -0.25, -1.5],
    [1.1, -0.25, -1.5],
  ];

  wheelPositions.forEach((pos) => {
    const wheel = new THREE.Mesh(wheelGeo, wheelMat);
    wheel.rotation.z = Math.PI / 2;
    wheel.position.set(pos[0], pos[1], pos[2]);
    car.add(wheel);
  });

  scene.add(car);
  camera.position.set(4, 3, 6);
  camera.lookAt(0, 0, 0);

  const animate = () => {
    requestAnimationFrame(animate);
    car.rotation.y += 0.005;
    renderer.render(scene, camera);
  };
  animate();
});

onUnmounted(() => {
  if (renderer) renderer.dispose();
});
</script>

<template>
  <div class="vehicle-3d card">
    <div class="card-header">
      <h3 class="card-title">🏎️ Digital Twin (3D State)</h3>
    </div>
    <div ref="container" class="canvas-container"></div>
    <div class="overlay-stats">
      <div class="stat">Yaw: 12.5°</div>
      <div class="stat">Pitch: 0.2°</div>
      <div class="stat">Roll: 1.1°</div>
    </div>
  </div>
</template>

<style scoped>
.vehicle-3d {
  position: relative;
  height: 400px;
  overflow: hidden;
}

.canvas-container {
  width: 100%;
  height: 100%;
}

.overlay-stats {
  position: absolute;
  bottom: 16px;
  left: 16px;
  display: flex;
  gap: 12px;
  pointer-events: none;
}

.stat {
  background: rgba(0, 0, 0, 0.5);
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 11px;
  color: var(--accent-blue);
  border: 1px solid rgba(56, 189, 248, 0.2);
}
</style>
