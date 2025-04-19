use crate::constants::return_values::*;
use crate::types::{char, void, HIDS, INT, UINT};

unsafe extern "C" {
    /// Allocates an image memory for an image having its dimensions defined by `width` and `height`
    /// and its color depth defined by `bitspixel`.
    ///
    /// The memory size is:
    /// ```txt
    /// size = [width * int((bitspixel + 7) / 8)] * height
    /// ```
    ///
    /// The line increment is calculated as:
    /// ```txt
    /// lineinc = width * int[(bitspixel + 7) / 8]
    /// ```
    ///
    /// To read out the line increment, you can use the [`is_GetImgMemPitch`] function.
    ///
    /// The starting address of the memory area is returned in `ppcMem`.
    /// `pid` returns an ID for the allocated memory. A newly allocated memory is not directly active,
    /// i.e. digitised images will not be stored immediately in this new memory. It must first be made
    /// active using [`is_SetImageMem`].
    ///
    /// The returned pointer must be write-protected and may not be altered because it will be used for
    /// all further `ImageMem` functions. To release the memory, you can use [`is_FreeImageMem`].
    ///
    /// In the Direct3D or OpenGL modes, image memory allocation is not necessary.
    ///
    /// `RGB16` and `RGB15` require the same amount of memory, but can be distinguished by the
    /// `bitspixel` parameter. For information on the bit depths of different color formats please
    /// refer to the
    /// [Appendix: Color and memory formats](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/sdk_allgemeines_farbformate.html) chapter.
    ///
    /// <div class="warning">
    /// In case the operating system is short of physical memory, today's OS versions swap individual
    /// areas of the RAM that have not been used for some time out to the slower hard disk. This can
    /// slow down image capture if more image memory has been allocated than can be provided by the
    /// RAM at a time.
    /// </div>
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `width` - Image width.
    /// * `height` - Image height.
    /// * `bitspixel` - Image bit depth (bits per pixel).
    /// * `ppcMem` - Returns the pointer to the memory starting address.
    /// * `pnMemId` - Returns the ID of this memory.
    ///
    /// # Return values
    /// * [`IS_CANT_ADD_TO_SEQUENCE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_OUT_OF_MEMORY`]
    /// * [`IS_SEQUENCE_BUF_ALREADY_LOCKED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_FreeImageMem`]
    /// * [`is_AddToSequence`]
    /// * [`is_SetImageMem`]
    /// * [`is_SetAllocatedImageMem`]
    /// * [`is_GetColorDepth`]
    /// * [`is_GetImgMemPitch`]
    ///
    /// # Documentation
    /// [is_AllocImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_allocimagemem.html)
    pub fn is_AllocImageMem(
        hCam: HIDS,
        width: INT,
        height: INT,
        bitspixel: INT,
        ppcMem: *mut *const char,
        pnMemId: *mut INT,
    ) -> INT;

    /// Makes the specified image memory the active memory.
    ///
    /// Only an active image memory can receive image data. When you call [`is_FreezeVideo`], the
    /// captured image is stored in the image buffer designated by `pcMem` and `nMemId`.
    /// For `pcMem`, you must pass a pointer which was created by [`is_AllocImageMem`], passing any
    /// other pointer will result in an error message. You may pass the same pointer multiple times.
    ///
    /// In the Direct3D or OpenGL modes, there is no need to set an image memory.
    ///
    /// If the specified image memory is set as active memory, a previously created sequence list is
    /// deleted and a new sequence list is created with this image memory. If image memories of the
    /// already existing sequence list are locked by the user and thus signaled as used, the image
    /// memories cannot be removed from the sequence list. So the sequence list is not deleted.
    /// Therefore, before calling [`is_SetImageMem`], **all** image memories locked using
    /// [`is_LockSeqBuf`] **have to be** unlocked using [`is_UnlockSeqBuf`] first.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pcMem` - Pointer to the starting position in the memory.
    /// * `nMemId` - ID of this memory.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SEQ_BUFFER_IS_LOCKED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_AllocImageMem`]
    /// * [`is_FreeImageMem`]
    /// * [`is_AddToSequence`]
    /// * [`is_SetAllocatedImageMem`]
    /// * [`is_GetColorDepth`]
    /// * [`is_GetImageMem`]
    /// * [`is_GetImagePitch`]
    ///
    /// # Documentation
    /// [is_SetImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_setimagemem.html)
    pub fn is_SetImageMem(hCam: HIDS, pcMem: *const char, nMemId: INT) -> INT;

    /// Releases an image memory that was allocated using [`is_AllocImageMem`] and removes it from
    /// the driver management.
    ///
    /// If the memory was not allocated using an SDK function, you need to call [`is_FreeImageMem`]
    /// as well. Otherwise, there may be errors when the driver keeps trying to access this memory.
    ///
    /// This does however not release the memory. So you need to make sure that the memory will be
    /// released again.
    ///
    /// If image memories are locked by the user and are thus signaled as used, the image memories
    /// cannot be released. Therefore, before calling [`is_FreeImageMem`], image memories locked
    /// using [`is_LockSeqBuf`] **has to be** unlocked using [`is_UnlockSeqBuf`] first.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pcMem` - Points to the starting address of the memory
    ///     (e.g. set in the [`is_AllocImageMem`] function)
    /// * `nMemId` - ID of this memory.
    ///
    /// # Return values
    /// * [`IS_CANT_ADD_TO_SEQUENCE`]
    /// * [`IS_CANT_CLEANUP_MEMORY`]
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_AllocImageMem`]
    ///
    /// # Documentation
    /// [is_FreeImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_freeimagemem.html)
    pub fn is_FreeImageMem(hCam: HIDS, pcMem: *const char, nMemId: INT) -> INT;

    /// Returns the pointer to the starting address of the active image memory.
    ///
    /// If you use ring buffering, [`is_GetImageMem`] returns the starting address of the image
    /// memory last used for image capturing.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pMem` - Pointer to the starting address of the image memory.
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_GetImageMemPitch`]
    /// * [`is_AllocImageMem`]
    /// * [`is_AddToSequence`]
    /// * [`is_SetImageMem`]
    /// * [`is_SetAllocatedImageMem`]
    ///
    /// # Documentation
    /// [is_GetImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getimagemem.html)
    pub fn is_GetImageMem(hCam: HIDS, pMem: *mut *const void) -> INT;

    /// Returns the pointer to the starting address of the active image memory.
    ///
    /// If a Direct3D mode is active and image memory was nevertheless allocated, the pointer to the
    /// image memory and its ID will be returned. However, in Direct3D mode, the image will not be
    /// copied automatically to this image memory.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `ppcMem` - Returns the pointer to the starting address of the active image memory.
    /// * `pnMemId` - Returns the ID of the active image memory.
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_ACTIVE_IMG_MEM`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_AllocImageMem`]
    /// * [`is_GetImageMem`]
    /// * [`is_SetImageMem`]
    /// * [`is_SetAllocatedImageMem`]
    ///
    /// # Documentation
    /// [is_GetActiveImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getactiveimagemem.html)
    pub fn is_GetActiveImageMem(hCam: HIDS, ppcMem: *mut *const char, pnMemId: *mut INT) -> INT;

    /// Returns the pointer to the starting address of the active image memory.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pMem` - Pointer to the starting address of the image memory as allocated by
    ///     [`is_AllocImageMem`].
    /// * `nMemId` - ID of the image memory as allocated by [`is_AllocImageMem`].
    /// * `pnX` - Returns the width used to define the image memory.
    ///     You can also pass [`NULL`] instead.
    /// * `pnY` - Returns the height used to define the image memory.
    ///     You can also pass [`NULL`] instead.
    /// * `pnBits` - Returns the bit width used to define the image memory.
    ///     You can also pass [`NULL`] instead.
    /// * `pnPitch` - Returns the line increment of the image memory.
    ///     You can also pass [`NULL`] instead.
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_AllocImageMem`]
    /// * [`is_SetImageMem`]
    /// * [`is_SetAllocatedImageMem`]
    /// * [`is_GetColorDepth`]
    ///
    /// # Documentation
    /// [is_InquireImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_inquireimagemem.html)
    pub fn is_InquireImageMem(
        hCam: HIDS,
        pcMem: *const char,
        nMemId: INT,
        pnX: *mut INT,
        pnY: *mut INT,
        pnBits: *mut INT,
        pnPitch: *mut INT,
    ) -> INT;

    /// Returns the line increment (in bytes) of the active image memory.
    ///
    /// The line increment is defined as the number of bytes from the beginning of a line to the
    /// beginning of the next line. The line increment is calculated as:
    /// ```txt
    /// lineinc = width * int[(bitspixel + 7) / 8]
    /// ```
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pPitch` - Pointer to the variable containing the line increment
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_GetImageMem`]
    /// * [`is_AllocImageMem`]
    /// * [`is_AddToSequence`]
    /// * [`is_SetImageMem`]
    /// * [`is_SetAllocatedImageMem`]
    ///
    /// # Documentation
    /// [is_GetImageMemPitch](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getimagemempitch.html)
    pub fn is_GetImageMemPitch(hCam: HIDS, pPitch: *mut INT) -> INT;

    /// Make a memory allocated by a user the active memory for storing digitized images in it.
    ///
    /// The allocated memory must be large enough and must always be locked globally.
    ///
    /// Depending on the selected image format you need more than one byte per pixel for
    /// image memory:
    /// ```rust
    /// use crate::ueye_sys::types::UINT;
    /// let width: UINT = 1920;
    /// let height: UINT = 1080;
    /// let bitspixel: UINT = 1;
    ///
    /// let uBytesPerPixel: UINT = (bitspixel+7)/8;
    /// let uImageSize: UINT = width * height * uBytesPerPixel;
    /// ```
    ///
    /// You can call the [`is_AddToSequence`] function to add a memory which was set using
    /// [`is_SetAllocatedImageMem`] to a sequence.
    ///
    /// The address of this memory will be passed to the _uEye_ driver. For this, you can use the
    /// [`is_SetAllocatedImageMem`] function. In addition, you need to specify the image size,
    /// just as you do when calling [`is_AllocImageMem`]. The returned memory ID is required by
    /// other functions for memory access.
    ///
    /// The memory area must be removed from the driver management again using the
    /// [`is_FreeImageMem`] function. Please note that this does not release the memory. You then
    /// need to make sure that the memory will be released again.
    ///
    /// After [`is_SetAllocatedImageMem`] you must call [`is_SetImageMem`] or [`is_AddToSequence`]
    /// in order that the image caption can be carried out in the image memory.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `width` - Image width.
    /// * `height` - Image height.
    /// * `bitspixel` - Image color depth (bits per pixel).
    /// * `pcMem` - Pointer to the starting address of the allocated memory.
    /// * `pnMemId` - Returns the ID of this memory.
    ///
    /// # Return values
    /// * [`IS_CANT_ADD_TO_SEQUENCE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_OUT_OF_MEMORY`]
    /// * [`IS_SEQUENCE_BUF_ALREADY_LOCKED`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_AllocImageMem`]
    /// * [`is_FreeImageMem`]
    /// * [`is_AddToSequence`]
    /// * [`is_SetImageMem`]
    /// * [`is_GetColorDepth`]
    /// * [`is_GetImageMemPitch`]
    ///
    /// # Documentation
    /// [is_GetActiveImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_getactiveimagemem.html)
    pub fn is_SetAllocatedImageMem(
        hCam: HIDS,
        width: INT,
        height: INT,
        bitspixel: INT,
        pcMem: *const char,
        pnMemId: *mut INT,
    ) -> INT;

    /// Copies the contents of the image memory described by `pcMemSrc` and `nMemId` to the memory
    /// area to whose starting address `pcMemDst` points.
    ///
    /// The allocated memory must be large enough to accommodate the entire image in its current
    /// format (bits per pixel).
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pcMemSrc` - Pointer to the image memory.
    /// * `nMemId` - ID of this image memory.
    /// * `pcMemDst` - Pointer to the destination memory to copy the image to.
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related function
    /// * [`is_AllocImageMem`]
    /// * [`is_SetAllocatedImageMem`]
    ///
    /// # Documentation
    /// [is_CopyImageMem](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_copyimagemem.html)
    pub fn is_CopyImageMem(
        hCam: HIDS,
        pcMemSrc: *const char,
        nMemId: INT,
        pcMemDst: *const char,
    ) -> INT;

    /// Copies the contents of the image memory described by `pcMemSrc` and `nMemId` to the memory
    /// area to whose starting address `pcMemDst` points.
    ///
    /// The function only copies the number of lines indicated by `nLines`.
    ///
    /// The allocated memory must be large enough to accommodate the in `nLines` given number of
    /// image lines considering the image width and format (Bits per Pixel).
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pcMemSrc` - Pointer to the image memory.
    /// * `nMemId` - ID of this image memory.
    /// * `nLines` - Number of lines to be copied.
    /// * `pcMemDst` - Pointer to the destination memory to copy the image to.
    ///
    /// # Return values
    /// * [`IS_CANT_COMMUNICATE_WITH_DRIVER`]
    /// * [`IS_CANT_OPEN_DEVICE`]
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_INVALID_MEMORY_POINTER`]
    /// * [`IS_INVALID_PARAMETER`]
    /// * [`IS_IO_REQUEST_FAILED`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_AllocImageMem`]
    /// * [`is_SetAllocatedImageMem`]
    ///
    /// # Documentation
    /// [is_CopyImageMemLines](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_copyimagememlines.html)
    pub fn is_CopyImageMemLines(
        hCam: HIDS,
        pcMemSrc: *const char,
        nMemId: INT,
        nLines: INT,
        pcMemDst: *const char,
    ) -> INT;

    /// Adds an image memory to the list of image memories used for ring buffering.
    ///
    /// The image memory must have been previously requested using [`is_AllocImageMem`].
    /// Using the [`is_SetAllocatedImageMem`] function, you can set a memory that has been
    /// allocated before as image memory. Image memories that are used for ring buffering must all
    /// have been allocated with the same color depth (bits per pixel).
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    /// * `pcMem` - Pointer to image memory.
    /// * `nMemId` - Image memory ID.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_AllocImageMem`]
    /// * [`is_ClearSequence`]
    /// * [`is_ImageQueue`]
    /// * [`is_SetImageMem`]
    /// * [`is_SetAllocatedImageMem`]
    ///
    /// # Documentation
    /// [is_AddToSequence](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_addtosequence.html)
    pub fn is_AddToSequence(hCam: HIDS, pcMem: *const char, nMemId: INT) -> INT;

    /// Removes all image memories from the sequence list that were added using
    /// [`is_AddToSequence`].
    ///
    /// After a call of [`is_ClearSequence`], there is no more active image memory. To make an
    /// image memory the active memory, call [`is_SetImageMem`].
    ///
    /// If image memories are locked by the user and are thus signaled as used, the image memories
    /// cannot be removed from the sequence list. Therefore, before calling [`is_ClearSequence`],
    /// all image memories locked using [`is_LockSeqBuf`] have to be unlocked using
    /// [`is_UnlockSeqBuf`] first.
    ///
    /// # Input parameters
    /// * `hCam` - Camera handle.
    ///
    /// # Return values
    /// * [`IS_INVALID_CAMERA_HANDLE`]
    /// * [`IS_NO_SUCCESS`]
    /// * [`IS_SEQ_BUFFER_IS_LOCKED`]
    /// * [`IS_SEQUENCE_LIST_EMPTY`]
    /// * [`IS_SUCCESS`]
    ///
    /// # Related functions
    /// * [`is_AllocImageMem`]
    /// * [`is_FreeImageMem`]
    /// * [`is_SetImageMem`]
    ///
    /// # Documentation
    /// [is_ClearSequence](https://www.1stvision.com/cameras/IDS/IDS-manuals/uEye_Manual/is_clearsequence.html)
    pub fn is_ClearSequence(hCam: HIDS) -> INT;
}
