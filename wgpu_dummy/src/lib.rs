use std::future::Ready;
use std::num::NonZero;

use wgpu::context::*;
use wgpu::{CompilationInfo, Error, RequestDeviceError};

#[derive(Debug)]
pub struct DummyInstance();

#[derive(Debug)]
pub struct DummyId();

impl From<DummyId> for ObjectId {
    fn from(value: DummyId) -> Self {
        unsafe { Self::new(NonZero::new_unchecked(1), NonZero::new_unchecked(1)) }
    }
}

impl From<ObjectId> for DummyId {
    fn from(value: ObjectId) -> Self {
        Self()
    }
}

impl Context for DummyInstance {
    type AdapterId = DummyId;

    type AdapterData = ();

    type DeviceId = DummyId;

    type DeviceData = ();

    type QueueId = DummyId;

    type QueueData = ();

    type ShaderModuleId = DummyId;

    type ShaderModuleData = ();

    type BindGroupLayoutId = DummyId;

    type BindGroupLayoutData = ();

    type BindGroupId = DummyId;

    type BindGroupData = ();

    type TextureViewId = DummyId;

    type TextureViewData = ();

    type SamplerId = DummyId;

    type SamplerData = ();

    type BufferId = DummyId;

    type BufferData = ();

    type TextureId = DummyId;

    type TextureData = ();

    type QuerySetId = DummyId;

    type QuerySetData = ();

    type PipelineLayoutId = DummyId;

    type PipelineLayoutData = ();

    type RenderPipelineId = DummyId;

    type RenderPipelineData = ();

    type ComputePipelineId = DummyId;

    type ComputePipelineData = ();

    type PipelineCacheId = DummyId;

    type PipelineCacheData = ();

    type CommandEncoderId = DummyId;

    type CommandEncoderData = ();

    type ComputePassId = DummyId;

    type ComputePassData = ();

    type RenderPassId = DummyId;

    type RenderPassData = ();

    type CommandBufferId = DummyId;

    type CommandBufferData = ();

    type RenderBundleEncoderId = DummyId;

    type RenderBundleEncoderData = ();

    type RenderBundleId = DummyId;

    type RenderBundleData = ();

    type SurfaceId = DummyId;

    type SurfaceData = ();

    type SurfaceOutputDetail = ();

    type SubmissionIndexData = ();

    type RequestAdapterFuture = Ready<Option<(Self::AdapterId, Self::AdapterData)>>;

    type RequestDeviceFuture = Ready<
        Result<
            (
                Self::DeviceId,
                Self::DeviceData,
                Self::QueueId,
                Self::QueueData,
            ),
            RequestDeviceError,
        >,
    >;

    type PopErrorScopeFuture = Ready<Option<Error>>;

    type CompilationInfoFuture = Ready<CompilationInfo>;

    fn init(instance_desc: wgt::InstanceDescriptor) -> Self {
        todo!()
    }

    unsafe fn instance_create_surface(
        &self,
        target: wgpu::SurfaceTargetUnsafe,
    ) -> Result<(Self::SurfaceId, Self::SurfaceData), wgpu::CreateSurfaceError> {
        todo!()
    }

    fn instance_request_adapter(
        &self,
        options: &wgpu::RequestAdapterOptions<'_, '_>,
    ) -> Self::RequestAdapterFuture {
        todo!()
    }

    fn adapter_request_device(
        &self,
        adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
        desc: &wgpu::DeviceDescriptor<'_>,
        trace_dir: Option<&std::path::Path>,
    ) -> Self::RequestDeviceFuture {
        todo!()
    }

    fn instance_poll_all_devices(&self, force_wait: bool) -> bool {
        todo!()
    }

    fn adapter_is_surface_supported(
        &self,
        adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
        surface: &Self::SurfaceId,
        surface_data: &Self::SurfaceData,
    ) -> bool {
        todo!()
    }

    fn adapter_features(
        &self,
        adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
    ) -> wgt::Features {
        todo!()
    }

    fn adapter_limits(
        &self,
        adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
    ) -> wgt::Limits {
        todo!()
    }

    fn adapter_downlevel_capabilities(
        &self,
        adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
    ) -> wgt::DownlevelCapabilities {
        todo!()
    }

    fn adapter_get_info(
        &self,
        adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
    ) -> wgt::AdapterInfo {
        todo!()
    }

    fn adapter_get_texture_format_features(
        &self,
        adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
        format: wgt::TextureFormat,
    ) -> wgt::TextureFormatFeatures {
        todo!()
    }

    fn adapter_get_presentation_timestamp(
        &self,
        adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
    ) -> wgt::PresentationTimestamp {
        todo!()
    }

    fn surface_get_capabilities(
        &self,
        surface: &Self::SurfaceId,
        surface_data: &Self::SurfaceData,
        adapter: &Self::AdapterId,
        adapter_data: &Self::AdapterData,
    ) -> wgt::SurfaceCapabilities {
        todo!()
    }

    fn surface_configure(
        &self,
        surface: &Self::SurfaceId,
        surface_data: &Self::SurfaceData,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        config: &wgpu::SurfaceConfiguration,
    ) {
        todo!()
    }

    fn surface_get_current_texture(
        &self,
        surface: &Self::SurfaceId,
        surface_data: &Self::SurfaceData,
    ) -> (
        Option<Self::TextureId>,
        Option<Self::TextureData>,
        wgt::SurfaceStatus,
        Self::SurfaceOutputDetail,
    ) {
        todo!()
    }

    fn surface_present(&self, texture: &Self::TextureId, detail: &Self::SurfaceOutputDetail) {
        todo!()
    }

    fn surface_texture_discard(
        &self,
        texture: &Self::TextureId,
        detail: &Self::SurfaceOutputDetail,
    ) {
        todo!()
    }

    fn device_features(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
    ) -> wgt::Features {
        todo!()
    }

    fn device_limits(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
    ) -> wgt::Limits {
        todo!()
    }

    fn device_downlevel_properties(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
    ) -> wgt::DownlevelCapabilities {
        todo!()
    }

    fn device_create_shader_module(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: wgpu::ShaderModuleDescriptor<'_>,
        shader_bound_checks: wgt::ShaderBoundChecks,
    ) -> (Self::ShaderModuleId, Self::ShaderModuleData) {
        todo!()
    }

    unsafe fn device_create_shader_module_spirv(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::ShaderModuleDescriptorSpirV<'_>,
    ) -> (Self::ShaderModuleId, Self::ShaderModuleData) {
        todo!()
    }

    fn device_create_bind_group_layout(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::BindGroupLayoutDescriptor<'_>,
    ) -> (Self::BindGroupLayoutId, Self::BindGroupLayoutData) {
        todo!()
    }

    fn device_create_bind_group(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::BindGroupDescriptor<'_>,
    ) -> (Self::BindGroupId, Self::BindGroupData) {
        todo!()
    }

    fn device_create_pipeline_layout(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::PipelineLayoutDescriptor<'_>,
    ) -> (Self::PipelineLayoutId, Self::PipelineLayoutData) {
        todo!()
    }

    fn device_create_render_pipeline(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::RenderPipelineDescriptor<'_>,
    ) -> (Self::RenderPipelineId, Self::RenderPipelineData) {
        todo!()
    }

    fn device_create_compute_pipeline(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::ComputePipelineDescriptor<'_>,
    ) -> (Self::ComputePipelineId, Self::ComputePipelineData) {
        todo!()
    }

    unsafe fn device_create_pipeline_cache(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::PipelineCacheDescriptor<'_>,
    ) -> (Self::PipelineCacheId, Self::PipelineCacheData) {
        todo!()
    }

    fn device_create_buffer(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::BufferDescriptor<'_>,
    ) -> (Self::BufferId, Self::BufferData) {
        todo!()
    }

    fn device_create_texture(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::TextureDescriptor<'_>,
    ) -> (Self::TextureId, Self::TextureData) {
        todo!()
    }

    fn device_create_sampler(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::SamplerDescriptor<'_>,
    ) -> (Self::SamplerId, Self::SamplerData) {
        todo!()
    }

    fn device_create_query_set(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::QuerySetDescriptor<'_>,
    ) -> (Self::QuerySetId, Self::QuerySetData) {
        todo!()
    }

    fn device_create_command_encoder(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::CommandEncoderDescriptor<'_>,
    ) -> (Self::CommandEncoderId, Self::CommandEncoderData) {
        todo!()
    }

    fn device_create_render_bundle_encoder(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        desc: &wgpu::RenderBundleEncoderDescriptor<'_>,
    ) -> (Self::RenderBundleEncoderId, Self::RenderBundleEncoderData) {
        todo!()
    }

    fn device_make_invalid(&self, device: &Self::DeviceId, device_data: &Self::DeviceData) {
        todo!()
    }

    fn device_drop(&self, device: &Self::DeviceId, device_data: &Self::DeviceData) {
        todo!()
    }

    fn device_set_device_lost_callback(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        device_lost_callback: DeviceLostCallback,
    ) {
        todo!()
    }

    fn device_destroy(&self, device: &Self::DeviceId, device_data: &Self::DeviceData) {
        todo!()
    }

    fn device_mark_lost(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        message: &str,
    ) {
        todo!()
    }

    fn queue_drop(&self, queue: &Self::QueueId, queue_data: &Self::QueueData) {
        todo!()
    }

    fn device_poll(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        maintain: wgpu::Maintain,
    ) -> wgt::MaintainResult {
        todo!()
    }

    fn device_on_uncaptured_error(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        handler: Box<dyn wgpu::UncapturedErrorHandler>,
    ) {
        todo!()
    }

    fn device_push_error_scope(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
        filter: wgpu::ErrorFilter,
    ) {
        todo!()
    }

    fn device_pop_error_scope(
        &self,
        device: &Self::DeviceId,
        device_data: &Self::DeviceData,
    ) -> Self::PopErrorScopeFuture {
        todo!()
    }

    fn buffer_map_async(
        &self,
        buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        mode: wgpu::MapMode,
        range: std::ops::Range<wgt::BufferAddress>,
        callback: BufferMapCallback,
    ) {
        todo!()
    }

    fn buffer_get_mapped_range(
        &self,
        buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        sub_range: std::ops::Range<wgt::BufferAddress>,
    ) -> Box<dyn BufferMappedRange> {
        todo!()
    }

    fn buffer_unmap(&self, buffer: &Self::BufferId, buffer_data: &Self::BufferData) {
        todo!()
    }

    fn shader_get_compilation_info(
        &self,
        shader: &Self::ShaderModuleId,
        shader_data: &Self::ShaderModuleData,
    ) -> Self::CompilationInfoFuture {
        todo!()
    }

    fn texture_create_view(
        &self,
        texture: &Self::TextureId,
        texture_data: &Self::TextureData,
        desc: &wgpu::TextureViewDescriptor<'_>,
    ) -> (Self::TextureViewId, Self::TextureViewData) {
        todo!()
    }

    fn surface_drop(&self, surface: &Self::SurfaceId, surface_data: &Self::SurfaceData) {
        todo!()
    }

    fn adapter_drop(&self, adapter: &Self::AdapterId, adapter_data: &Self::AdapterData) {
        todo!()
    }

    fn buffer_destroy(&self, buffer: &Self::BufferId, buffer_data: &Self::BufferData) {
        todo!()
    }

    fn buffer_drop(&self, buffer: &Self::BufferId, buffer_data: &Self::BufferData) {
        todo!()
    }

    fn texture_destroy(&self, texture: &Self::TextureId, texture_data: &Self::TextureData) {
        todo!()
    }

    fn texture_drop(&self, texture: &Self::TextureId, texture_data: &Self::TextureData) {
        todo!()
    }

    fn texture_view_drop(
        &self,
        texture_view: &Self::TextureViewId,
        texture_view_data: &Self::TextureViewData,
    ) {
        todo!()
    }

    fn sampler_drop(&self, sampler: &Self::SamplerId, sampler_data: &Self::SamplerData) {
        todo!()
    }

    fn query_set_drop(&self, query_set: &Self::QuerySetId, query_set_data: &Self::QuerySetData) {
        todo!()
    }

    fn bind_group_drop(
        &self,
        bind_group: &Self::BindGroupId,
        bind_group_data: &Self::BindGroupData,
    ) {
        todo!()
    }

    fn bind_group_layout_drop(
        &self,
        bind_group_layout: &Self::BindGroupLayoutId,
        bind_group_layout_data: &Self::BindGroupLayoutData,
    ) {
        todo!()
    }

    fn pipeline_layout_drop(
        &self,
        pipeline_layout: &Self::PipelineLayoutId,
        pipeline_layout_data: &Self::PipelineLayoutData,
    ) {
        todo!()
    }

    fn shader_module_drop(
        &self,
        shader_module: &Self::ShaderModuleId,
        shader_module_data: &Self::ShaderModuleData,
    ) {
        todo!()
    }

    fn command_encoder_drop(
        &self,
        command_encoder: &Self::CommandEncoderId,
        command_encoder_data: &Self::CommandEncoderData,
    ) {
        todo!()
    }

    fn command_buffer_drop(
        &self,
        command_buffer: &Self::CommandBufferId,
        command_buffer_data: &Self::CommandBufferData,
    ) {
        todo!()
    }

    fn render_bundle_drop(
        &self,
        render_bundle: &Self::RenderBundleId,
        render_bundle_data: &Self::RenderBundleData,
    ) {
        todo!()
    }

    fn compute_pipeline_drop(
        &self,
        pipeline: &Self::ComputePipelineId,
        pipeline_data: &Self::ComputePipelineData,
    ) {
        todo!()
    }

    fn render_pipeline_drop(
        &self,
        pipeline: &Self::RenderPipelineId,
        pipeline_data: &Self::RenderPipelineData,
    ) {
        todo!()
    }

    fn pipeline_cache_drop(
        &self,
        cache: &Self::PipelineCacheId,
        cache_data: &Self::PipelineCacheData,
    ) {
        todo!()
    }

    fn compute_pipeline_get_bind_group_layout(
        &self,
        pipeline: &Self::ComputePipelineId,
        pipeline_data: &Self::ComputePipelineData,
        index: u32,
    ) -> (Self::BindGroupLayoutId, Self::BindGroupLayoutData) {
        todo!()
    }

    fn render_pipeline_get_bind_group_layout(
        &self,
        pipeline: &Self::RenderPipelineId,
        pipeline_data: &Self::RenderPipelineData,
        index: u32,
    ) -> (Self::BindGroupLayoutId, Self::BindGroupLayoutData) {
        todo!()
    }

    fn command_encoder_copy_buffer_to_buffer(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        source: &Self::BufferId,
        source_data: &Self::BufferData,
        source_offset: wgt::BufferAddress,
        destination: &Self::BufferId,
        destination_data: &Self::BufferData,
        destination_offset: wgt::BufferAddress,
        copy_size: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn command_encoder_copy_buffer_to_texture(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        source: wgpu::ImageCopyBuffer<'_>,
        destination: wgpu::ImageCopyTexture<'_>,
        copy_size: wgt::Extent3d,
    ) {
        todo!()
    }

    fn command_encoder_copy_texture_to_buffer(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        source: wgpu::ImageCopyTexture<'_>,
        destination: wgpu::ImageCopyBuffer<'_>,
        copy_size: wgt::Extent3d,
    ) {
        todo!()
    }

    fn command_encoder_copy_texture_to_texture(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        source: wgpu::ImageCopyTexture<'_>,
        destination: wgpu::ImageCopyTexture<'_>,
        copy_size: wgt::Extent3d,
    ) {
        todo!()
    }

    fn command_encoder_begin_compute_pass(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        desc: &wgpu::ComputePassDescriptor<'_>,
    ) -> (Self::ComputePassId, Self::ComputePassData) {
        todo!()
    }

    fn command_encoder_begin_render_pass(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        desc: &wgpu::RenderPassDescriptor<'_>,
    ) -> (Self::RenderPassId, Self::RenderPassData) {
        todo!()
    }

    fn command_encoder_finish(
        &self,
        encoder: Self::CommandEncoderId,
        encoder_data: &mut Self::CommandEncoderData,
    ) -> (Self::CommandBufferId, Self::CommandBufferData) {
        todo!()
    }

    fn command_encoder_clear_texture(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        texture: &wgpu::Texture, // TODO: Decompose?
        subresource_range: &wgt::ImageSubresourceRange,
    ) {
        todo!()
    }

    fn command_encoder_clear_buffer(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        buffer: &wgpu::Buffer,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferAddress>,
    ) {
        todo!()
    }

    fn command_encoder_insert_debug_marker(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        label: &str,
    ) {
        todo!()
    }

    fn command_encoder_push_debug_group(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        label: &str,
    ) {
        todo!()
    }

    fn command_encoder_pop_debug_group(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
    ) {
        todo!()
    }

    fn command_encoder_write_timestamp(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        query_set: &Self::QuerySetId,
        query_set_data: &Self::QuerySetData,
        query_index: u32,
    ) {
        todo!()
    }

    fn command_encoder_resolve_query_set(
        &self,
        encoder: &Self::CommandEncoderId,
        encoder_data: &Self::CommandEncoderData,
        query_set: &Self::QuerySetId,
        query_set_data: &Self::QuerySetData,
        first_query: u32,
        query_count: u32,
        destination: &Self::BufferId,
        destination_data: &Self::BufferData,
        destination_offset: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn render_bundle_encoder_finish(
        &self,
        encoder: Self::RenderBundleEncoderId,
        encoder_data: Self::RenderBundleEncoderData,
        desc: &wgpu::RenderBundleDescriptor<'_>,
    ) -> (Self::RenderBundleId, Self::RenderBundleData) {
        todo!()
    }

    fn queue_write_buffer(
        &self,
        queue: &Self::QueueId,
        queue_data: &Self::QueueData,
        buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        offset: wgt::BufferAddress,
        data: &[u8],
    ) {
        todo!()
    }

    fn queue_validate_write_buffer(
        &self,
        queue: &Self::QueueId,
        queue_data: &Self::QueueData,
        buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        offset: wgt::BufferAddress,
        size: wgt::BufferSize,
    ) -> Option<()> {
        todo!()
    }

    fn queue_create_staging_buffer(
        &self,
        queue: &Self::QueueId,
        queue_data: &Self::QueueData,
        size: wgt::BufferSize,
    ) -> Option<Box<dyn QueueWriteBuffer>> {
        todo!()
    }

    fn queue_write_staging_buffer(
        &self,
        queue: &Self::QueueId,
        queue_data: &Self::QueueData,
        buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        offset: wgt::BufferAddress,
        staging_buffer: &dyn QueueWriteBuffer,
    ) {
        todo!()
    }

    fn queue_write_texture(
        &self,
        queue: &Self::QueueId,
        queue_data: &Self::QueueData,
        texture: wgpu::ImageCopyTexture<'_>,
        data: &[u8],
        data_layout: wgt::ImageDataLayout,
        size: wgt::Extent3d,
    ) {
        todo!()
    }

    fn queue_submit<I: Iterator<Item = (Self::CommandBufferId, Self::CommandBufferData)>>(
        &self,
        queue: &Self::QueueId,
        queue_data: &Self::QueueData,
        command_buffers: I,
    ) -> Self::SubmissionIndexData {
        todo!()
    }

    fn queue_get_timestamp_period(
        &self,
        queue: &Self::QueueId,
        queue_data: &Self::QueueData,
    ) -> f32 {
        todo!()
    }

    fn queue_on_submitted_work_done(
        &self,
        queue: &Self::QueueId,
        queue_data: &Self::QueueData,
        callback: SubmittedWorkDoneCallback,
    ) {
        todo!()
    }

    fn device_start_capture(&self, device: &Self::DeviceId, device_data: &Self::DeviceData) {
        todo!()
    }

    fn device_stop_capture(&self, device: &Self::DeviceId, device_data: &Self::DeviceData) {
        todo!()
    }

    fn device_get_internal_counters(
        &self,
        device: &Self::DeviceId,
        _device_data: &Self::DeviceData,
    ) -> wgt::InternalCounters {
        todo!()
    }

    fn pipeline_cache_get_data(
        &self,
        cache: &Self::PipelineCacheId,
        cache_data: &Self::PipelineCacheData,
    ) -> Option<Vec<u8>> {
        todo!()
    }

    fn compute_pass_set_pipeline(
        &self,
        pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        pipeline: &Self::ComputePipelineId,
        pipeline_data: &Self::ComputePipelineData,
    ) {
        todo!()
    }

    fn compute_pass_set_bind_group(
        &self,
        pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        index: u32,
        bind_group: &Self::BindGroupId,
        bind_group_data: &Self::BindGroupData,
        offsets: &[wgt::DynamicOffset],
    ) {
        todo!()
    }

    fn compute_pass_set_push_constants(
        &self,
        pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        offset: u32,
        data: &[u8],
    ) {
        todo!()
    }

    fn compute_pass_insert_debug_marker(
        &self,
        pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        label: &str,
    ) {
        todo!()
    }

    fn compute_pass_push_debug_group(
        &self,
        pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        group_label: &str,
    ) {
        todo!()
    }

    fn compute_pass_pop_debug_group(
        &self,
        pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
    ) {
        todo!()
    }

    fn compute_pass_write_timestamp(
        &self,
        pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        query_set: &Self::QuerySetId,
        query_set_data: &Self::QuerySetData,
        query_index: u32,
    ) {
        todo!()
    }

    fn compute_pass_begin_pipeline_statistics_query(
        &self,
        pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        query_set: &Self::QuerySetId,
        query_set_data: &Self::QuerySetData,
        query_index: u32,
    ) {
        todo!()
    }

    fn compute_pass_end_pipeline_statistics_query(
        &self,
        pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
    ) {
        todo!()
    }

    fn compute_pass_dispatch_workgroups(
        &self,
        pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        x: u32,
        y: u32,
        z: u32,
    ) {
        todo!()
    }

    fn compute_pass_dispatch_workgroups_indirect(
        &self,
        pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn compute_pass_end(
        &self,
        pass: &mut Self::ComputePassId,
        pass_data: &mut Self::ComputePassData,
    ) {
        todo!()
    }

    fn render_bundle_encoder_set_pipeline(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        pipeline: &Self::RenderPipelineId,
        pipeline_data: &Self::RenderPipelineData,
    ) {
        todo!()
    }

    fn render_bundle_encoder_set_bind_group(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        index: u32,
        bind_group: &Self::BindGroupId,
        bind_group_data: &Self::BindGroupData,
        offsets: &[wgt::DynamicOffset],
    ) {
        todo!()
    }

    fn render_bundle_encoder_set_index_buffer(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        index_format: wgt::IndexFormat,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferSize>,
    ) {
        todo!()
    }

    fn render_bundle_encoder_set_vertex_buffer(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        slot: u32,
        buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferSize>,
    ) {
        todo!()
    }

    fn render_bundle_encoder_set_push_constants(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        stages: wgt::ShaderStages,
        offset: u32,
        data: &[u8],
    ) {
        todo!()
    }

    fn render_bundle_encoder_draw(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        vertices: std::ops::Range<u32>,
        instances: std::ops::Range<u32>,
    ) {
        todo!()
    }

    fn render_bundle_encoder_draw_indexed(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        indices: std::ops::Range<u32>,
        base_vertex: i32,
        instances: std::ops::Range<u32>,
    ) {
        todo!()
    }

    fn render_bundle_encoder_draw_indirect(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn render_bundle_encoder_draw_indexed_indirect(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn render_bundle_encoder_multi_draw_indirect(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
        count: u32,
    ) {
        todo!()
    }

    fn render_bundle_encoder_multi_draw_indexed_indirect(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
        count: u32,
    ) {
        todo!()
    }

    fn render_bundle_encoder_multi_draw_indirect_count(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
        count_buffer: &Self::BufferId,
        count_buffer_data: &Self::BufferData,
        count_buffer_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        todo!()
    }

    fn render_bundle_encoder_multi_draw_indexed_indirect_count(
        &self,
        encoder: &mut Self::RenderBundleEncoderId,
        encoder_data: &mut Self::RenderBundleEncoderData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
        count_buffer: &Self::BufferId,
        count_buffer_data: &Self::BufferData,
        count_buffer_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        todo!()
    }

    fn render_pass_set_pipeline(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        pipeline: &Self::RenderPipelineId,
        pipeline_data: &Self::RenderPipelineData,
    ) {
        todo!()
    }

    fn render_pass_set_bind_group(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        index: u32,
        bind_group: &Self::BindGroupId,
        bind_group_data: &Self::BindGroupData,
        offsets: &[wgt::DynamicOffset],
    ) {
        todo!()
    }

    fn render_pass_set_index_buffer(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        index_format: wgt::IndexFormat,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferSize>,
    ) {
        todo!()
    }

    fn render_pass_set_vertex_buffer(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        slot: u32,
        buffer: &Self::BufferId,
        buffer_data: &Self::BufferData,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferSize>,
    ) {
        todo!()
    }

    fn render_pass_set_push_constants(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        stages: wgt::ShaderStages,
        offset: u32,
        data: &[u8],
    ) {
        todo!()
    }

    fn render_pass_draw(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        vertices: std::ops::Range<u32>,
        instances: std::ops::Range<u32>,
    ) {
        todo!()
    }

    fn render_pass_draw_indexed(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        indices: std::ops::Range<u32>,
        base_vertex: i32,
        instances: std::ops::Range<u32>,
    ) {
        todo!()
    }

    fn render_pass_draw_indirect(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn render_pass_draw_indexed_indirect(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
    ) {
        todo!()
    }

    fn render_pass_multi_draw_indirect(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
        count: u32,
    ) {
        todo!()
    }

    fn render_pass_multi_draw_indexed_indirect(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
        count: u32,
    ) {
        todo!()
    }

    fn render_pass_multi_draw_indirect_count(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
        count_buffer: &Self::BufferId,
        count_buffer_data: &Self::BufferData,
        count_buffer_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        todo!()
    }

    fn render_pass_multi_draw_indexed_indirect_count(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        indirect_buffer: &Self::BufferId,
        indirect_buffer_data: &Self::BufferData,
        indirect_offset: wgt::BufferAddress,
        count_buffer: &Self::BufferId,
        count_buffer_data: &Self::BufferData,
        count_buffer_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        todo!()
    }

    fn render_pass_set_blend_constant(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        color: wgt::Color,
    ) {
        todo!()
    }

    fn render_pass_set_scissor_rect(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) {
        todo!()
    }

    fn render_pass_set_viewport(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) {
        todo!()
    }

    fn render_pass_set_stencil_reference(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        reference: u32,
    ) {
        todo!()
    }

    fn render_pass_insert_debug_marker(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        label: &str,
    ) {
        todo!()
    }

    fn render_pass_push_debug_group(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        group_label: &str,
    ) {
        todo!()
    }

    fn render_pass_pop_debug_group(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
    ) {
        todo!()
    }

    fn render_pass_write_timestamp(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        query_set: &Self::QuerySetId,
        query_set_data: &Self::QuerySetData,
        query_index: u32,
    ) {
        todo!()
    }

    fn render_pass_begin_occlusion_query(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        query_index: u32,
    ) {
        todo!()
    }

    fn render_pass_end_occlusion_query(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
    ) {
        todo!()
    }

    fn render_pass_begin_pipeline_statistics_query(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        query_set: &Self::QuerySetId,
        query_set_data: &Self::QuerySetData,
        query_index: u32,
    ) {
        todo!()
    }

    fn render_pass_end_pipeline_statistics_query(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
    ) {
        todo!()
    }

    fn render_pass_execute_bundles(
        &self,
        pass: &mut Self::RenderPassId,
        pass_data: &mut Self::RenderPassData,
        render_bundles: &mut dyn Iterator<Item = (Self::RenderBundleId, &Self::RenderBundleData)>,
    ) {
        todo!()
    }

    fn render_pass_end(&self, pass: &mut Self::RenderPassId, pass_data: &mut Self::RenderPassData) {
        todo!()
    }
}
