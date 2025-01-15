<template>
    <div class="drawer-side block w-auto pointer-events-auto visible static h-[calc(100dvh-180px)] shadow-lg">
        <aside class="bg-base-200 w-80 h-full" style="transform: translate(0);">
            <ul class="menu px-4 py-0">
                <li>
                    <details open>
                        <summary class="group">
                            Albums
                        </summary>
                        <ul>
                            <li>
                                <RouterLink to="/album/all"><span>All</span></RouterLink>
                            </li>
                            <li>
                                <RouterLink to="/album/recentlyAdded"><span>Recently Added</span></RouterLink>
                            </li>
                        </ul>
                    </details>
                </li>
                <li>
                    <details>
                        <summary class="group">
                            Songs
                        </summary>
                        <ul>
                            <li>
                                <RouterLink to="/song/all"><span>All </span></RouterLink>
                            </li>
                            <li>
                                <RouterLink to="/song/recentlyAdded"><span>Recently Added</span></RouterLink>
                            </li>
                        </ul>
                    </details>
                </li>
            </ul>
        </aside>
    </div>
</template>

<script setup lang="ts">

</script>