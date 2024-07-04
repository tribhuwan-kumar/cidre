use crate::{define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTLIndirectRenderCommand")]
    pub IndirectRenderCmd(ns::Id)
);

impl IndirectRenderCmd {
    /// Sets the render pipeline state object used by the command.
    ///
    /// If you created the indirect command buffer with inheritPipelineState set to true,
    /// do not call this method. The command gets the pipeline state object from the parent
    /// encoder when you execute the command.
    ///
    /// If you created the indirect command buffer with inheritPipelineState set to false,
    /// you must set the pipeline state prior to encoding the drawing command.
    /// TODO: if it throws wrap it in try - catch
    #[objc::msg_send(setRenderPipelineState:)]
    pub fn set_render_ps(&mut self, ps: &mtl::RenderPipelineState);

    /// Sets a vertex buffer argument for the command.
    ///
    /// If you created the indirect command buffer with inheritBuffers set to true,
    /// do not call this method. The command gets the arguments from the parent encoder
    /// when you execute the command.
    ///
    /// If you need to pass other kinds of parameters to your shader, such as textures
    /// and samplers, create an argument buffer and pass it to the shader using this method.
    #[objc::msg_send(setVertexBuffer:offset:atIndex:)]
    pub fn set_vertex_buf_at(&mut self, buf: &mtl::Buf, offset: usize, index: usize);

    /// Sets a vertex buffer argument for the command.
    ///
    /// If you created the indirect command buffer with inheritBuffers set to true,
    /// do not call this method. The command gets the arguments from the parent encoder
    /// when you execute the command.
    ///
    /// If you need to pass other kinds of parameters to your shader, such as textures
    /// and samplers, create an argument buffer and pass it to the shader using this method.
    #[objc::msg_send(setFragmentBuffer:offset:atIndex:)]
    pub fn set_fragment_buf_at(&mut self, buf: &mtl::Buf, offset: usize, index: usize);

    /// Encodes a command to render a number of instances of tessellated patches.
    ///
    /// The command generated by this method is equivalent to calling
    /// setTessellationFactorBuffer:offset:instanceStride: followed by
    ///  drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:.
    #[objc::msg_send(drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:)]
    pub fn draw_patches(
        &self,
        number_of_patch_ctrl_points: usize,
        patch_start: usize,
        patch_count: usize,
        patch_index_buf: &mtl::Buf,
        patch_index_buf_offset: usize,
        instance_count: usize,
        base_instance: usize,
        tessellation_factor_buf: &mtl::Buf,
        tessellation_factor_buf_offset: usize,
        tessellation_factor_buf_instance_stride: usize,
    );

    /// Encodes a command to render a number of instances of tessellated patches,
    /// using a control point index buffer.
    #[objc::msg_send(drawIndexedPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:)]
    pub fn draw_indexed_patches(
        &self,
        number_of_patch_ctrl_points: usize,
        patch_start: usize,
        patch_count: usize,
        patch_index_buf: Option<&mtl::Buf>,
        patch_index_buff_offset: usize,
        ctrl_point_index_buf: &mtl::Buf,
        ctrl_point_index_buf_offset: usize,
        instance_count: usize,
        base_instance: usize,
        tessellation_factor_buf: &mtl::Buf,
        tessellation_factor_buf_offset: usize,
        tessellation_factor_buf_instance_stride: usize,
    );

    /// Encodes a command to render a number of instances of primitives using vertex data
    /// in contiguous array elements, starting from the base instance.
    #[objc::msg_send(drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:usize)]
    pub fn draw_primitives(
        &self,
        primitive_type: mtl::Primitive,
        vertex_start: usize,
        vertex_count: usize,
        instance_count: usize,
        base_instance: usize,
    );

    /// Encodes a command to render a number of instances of primitives using an index list specified in a buffer,
    /// starting from the base vertex of the base instance.
    #[objc::msg_send(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:)]
    pub fn draw_indexed_primitives_index_type_index_count_instance_count(
        &self,
        primitive_type: mtl::Primitive,
        index_count: usize,
        index_type: mtl::IndexType,
        index_buf: &mtl::Buf,
        index_buf_offset: usize,
        instance_count: usize,
        base_vertex: isize,
        base_instance: usize,
    );

    #[inline]
    pub fn draw_indexed_triangles_u16(
        &mut self,
        index_buffer: &mtl::Buf,
        index_range: &std::ops::Range<usize>,
        instance_range: &std::ops::Range<usize>,
    ) {
        self.draw_indexed_primitives_index_type_index_count_instance_count(
            mtl::Primitive::Triangle,
            index_range.len(),
            mtl::IndexType::U16,
            index_buffer,
            index_range.start * std::mem::size_of::<u16>(),
            instance_range.len(),
            0, // base vertex,
            instance_range.start,
        );
    }

    #[inline]
    pub fn draw_indexed_triangles_u32(
        &mut self,
        index_buffer: &mtl::Buf,
        index_range: &std::ops::Range<usize>,
        instance_range: &std::ops::Range<usize>,
    ) {
        self.draw_indexed_primitives_index_type_index_count_instance_count(
            mtl::Primitive::Triangle,
            index_range.len(),
            mtl::IndexType::U32,
            index_buffer,
            index_range.start * std::mem::size_of::<u32>(),
            instance_range.len(),
            0, // base vertex,
            instance_range.start,
        );
    }

    /// Resets the command to its default state.
    ///
    /// A command that has been reset loses any state that you previously
    /// set and does nothing when executed.
    #[objc::msg_send(reset)]
    pub fn reset(&mut self);
}

define_obj_type!(
    #[doc(alias = "MTLIndirectComputeCommand")]
    pub IndirectComputeCmd(ns::Id)
);

impl IndirectComputeCmd {
    #[objc::msg_send(setComputePipelineState:)]
    pub fn set_compute_ps(&mut self, ps: &mtl::ComputePipelineState);

    #[objc::msg_send(setKernelBuffer:offset:atIndex:)]
    pub fn set_kernel_buf_at(&mut self, buf: &mtl::Buf, offset: usize, index: usize);

    #[objc::msg_send(concurrentDispatchThreadgroups:threadsPerThreadgroup:)]
    pub fn concurrent_dispatch_thread_groups(
        &mut self,
        threadgroups_per_grid: mtl::Size,
        threads_per_threadgroup: mtl::Size,
    );

    #[objc::msg_send(concurrentDispatchThreads:threadsPerThreadgroup:)]
    pub fn concurrent_dispatch_threads(
        &mut self,
        threads_per_grid: mtl::Size,
        threads_per_threadgroup: mtl::Size,
    );

    #[objc::msg_send(setBarrier)]
    pub fn set_barrier(&mut self);

    #[objc::msg_send(clearBarrier)]
    pub fn clear_barrier(&mut self);

    #[objc::msg_send(setImageblockWidth:height:)]
    pub fn set_image_block_size(&mut self, width: usize, height: usize);

    #[objc::msg_send(reset)]
    pub fn reset(&mut self);

    #[objc::msg_send(setThreadgroupMemoryLength:atIndex:)]
    pub fn set_threadgroup_memory_len_at(&mut self, length: usize, index: usize);

    #[objc::msg_send(setStageInRegion:)]
    pub fn set_stage_in_region(&mut self, region: mtl::Region);
}
