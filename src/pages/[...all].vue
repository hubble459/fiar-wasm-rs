<template>
    <el-row>
        <el-col>
            <el-result
                icon="warning"
                :title="t('exception.not_found')"
                :sub-title="t('exception.not_found_sub')"
            >
                <template #extra>
                    <el-button
                        type="primary"
                        @click="goBack"
                    >
                        {{ t('global.back') }}
                    </el-button>
                </template>
            </el-result>
        </el-col>
    </el-row>
</template>

<script setup lang="ts">
    const { t } = useI18n();

    const router = useRouter();
    function goBack() {
        router.push({
            name: 'home',
        });
    }
</script>
