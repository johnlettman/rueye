# uEye API Coverage
## Functions
* [ ] `is_CaptureStatus` (`HIDS hCam, UINT nCommand, void *pParam, UINT nSizeOfParam`) ⇝ `IDSEXP`
* [ ] `is_WaitEvent` (`HIDS hCam, INT which, INT nTimeout`) ⇝ `IDSEXPDEP`
* [ ] `is_SetSaturation` (`HIDS hCam, INT ChromU, INT ChromV`) ⇝ `IDSEXP`
* [x] `is_PrepareStealVideo` (`HIDS hCam, int Mode, ULONG StealColorMode`) ⇝ `IDSEXP`
* [ ] `is_GetNumberOfDevices` (`void`) ⇝ `IDSEXP`
* [x] `is_StopLiveVideo` (`HIDS hCam, INT Wait`) ⇝ `IDSEXP`
* [x] `is_FreezeVideo` (`HIDS hCam, INT Wait`) ⇝ `IDSEXP`
* [x] `is_CaptureVideo` (`HIDS hCam, INT Wait`) ⇝ `IDSEXP`
* [x] `is_IsVideoFinish` (`HIDS hCam, INT* pValue`) ⇝ `IDSEXP`
* [x] `is_HasVideoStarted` (`HIDS hCam, BOOL* pbo`) ⇝ `IDSEXP`
* [x] `is_AllocImageMem` (`HIDS hCam, INT width, INT height, INT bitspixel, char** ppcMem, int* pnMemId`) ⇝ `IDSEXP`
* [x] `is_SetImageMem` (`HIDS hCam, char* pcMem, int nMemId`) ⇝ `IDSEXP`
* [x] `is_FreeImageMem` (`HIDS hCam, char* pcMem, int nMemId`) ⇝ `IDSEXP`
* [x] `is_GetImageMem` (`HIDS hCam, VOID** pMem`) ⇝ `IDSEXP`
* [x] `is_GetActiveImageMem` (`HIDS hCam, char** ppcMem, int* pnMemId`) ⇝ `IDSEXP`
* [x] `is_InquireImageMem` (`HIDS hCam, char* pcMem, int nMemId, int* pnX, int* pnY, int* pnBits, int* pnPitch`) ⇝ `IDSEXP`
* [x] `is_GetImageMemPitch` (`HIDS hCam, INT* pPitch`) ⇝ `IDSEXP`
* [x] `is_SetAllocatedImageMem` (`HIDS hCam, INT width, INT height, INT bitspixel, char* pcMem, int* pnMemId`) ⇝ `IDSEXP`
* [x] `is_CopyImageMem` (`HIDS hCam, char* pcMemSrc, int nMemId, char* pcMemDst`) ⇝ `IDSEXP`
* [x] `is_CopyImageMemLines` (`HIDS hCam, char* pcMemSrc, int nMemId, int nLines, char* pcMemDst`) ⇝ `IDSEXP`
* [x] `is_AddToSequence` (`HIDS hCam, char* pcMem, INT nMemId`) ⇝ `IDSEXP`
* [x] `is_ClearSequence` (`HIDS hCam`) ⇝ `IDSEXP`
* [x] `is_GetActSeqBuf` (`HIDS hCam, INT* pnNum, char** ppcMem, char** ppcMemLast`) ⇝ `IDSEXP`
* [x] `is_LockSeqBuf` (`HIDS hCam, INT nMemId, char* pcMem`) ⇝ `IDSEXP`
* [x] `is_UnlockSeqBuf` (`HIDS hCam, INT nMemId, char* pcMem`) ⇝ `IDSEXP`
* [x] `is_GetError` (`HIDS hCam, INT* pErr, IS_CHAR** ppcErr`) ⇝ `IDSEXP`
* [x] `is_SetErrorReport` (`HIDS hCam, INT Mode`) ⇝ `IDSEXP`
* [x] `is_SetColorMode` (`HIDS hCam, INT Mode`) ⇝ `IDSEXP`
* [x] `is_GetColorDepth` (`HIDS hCam, INT* pnCol, INT* pnColMode`) ⇝ `IDSEXP`
* [x] `is_RenderBitmap` (`HIDS hCam, INT nMemId, HWND hwnd, INT nMode`) ⇝ `IDSEXP`
* [x] `is_SetDisplayMode` (`HIDS hCam, INT Mode`) ⇝ `IDSEXP`
* [x] `is_SetDisplayPos` (`HIDS hCam, INT x, INT y`) ⇝ `IDSEXP`
* [x] `is_SetHwnd` (`HIDS hCam, HWND hwnd`) ⇝ `IDSEXP`
* [x] `is_GetVsyncCount` (`HIDS hCam, long* pIntr, long* pActIntr`) ⇝ `IDSEXP`
* [ ] `is_GetDLLVersion` (`void`) ⇝ `IDSEXP`
* [ ] `is_InitEvent` (`HIDS hCam, HANDLE hEv, INT which`) ⇝ `IDSEXPDEP`
* [ ] `is_ExitEvent` (`HIDS hCam, INT which`) ⇝ `IDSEXPDEP`
* [ ] `is_EnableEvent` (`HIDS hCam, INT which`) ⇝ `IDSEXPDEP`
* [ ] `is_DisableEvent` (`HIDS hCam, INT which`) ⇝ `IDSEXPDEP`
* [ ] `is_SetExternalTrigger` (`HIDS hCam, INT nTriggerMode`) ⇝ `IDSEXP`
* [ ] `is_SetTriggerCounter` (`HIDS hCam, INT nValue`) ⇝ `IDSEXPDEP`
* [ ] `is_SetRopEffect` (`HIDS hCam, INT effect, INT param, INT reserved`) ⇝ `IDSEXP`
* [ ] `is_InitCamera` (`HIDS* phCam, HWND hWnd`) ⇝ `IDSEXP`
* [ ] `is_ExitCamera` (`HIDS hCam`) ⇝ `IDSEXP`
* [ ] `is_GetCameraInfo` (`HIDS hCam, PCAMINFO pInfo`) ⇝ `IDSEXP`
* [ ] `is_CameraStatus` (`HIDS hCam, INT nInfo, ULONG ulValue`) ⇝ `IDSEXPUL`
* [ ] `is_GetCameraType` (`HIDS hCam`) ⇝ `IDSEXP`
* [ ] `is_GetNumberOfCameras` (`INT* pnNumCams`) ⇝ `IDSEXP`
* [ ] `is_GetUsedBandwidth` (`HIDS hCam`) ⇝ `IDSEXP`
* [ ] `is_GetFrameTimeRange` (`HIDS hCam, double *min, double *max, double *intervall`) ⇝ `IDSEXP`
* [ ] `is_SetFrameRate` (`HIDS hCam, double FPS, double* newFPS`) ⇝ `IDSEXP`
* [ ] `is_GetFramesPerSecond` (`HIDS hCam, double *dblFPS`) ⇝ `IDSEXP`
* [ ] `is_GetSensorInfo` (`HIDS hCam, PSENSORINFO pInfo`) ⇝ `IDSEXP`
* [ ] `is_GetRevisionInfo` (`HIDS hCam, PREVISIONINFO prevInfo`) ⇝ `IDSEXP`
* [ ] `is_EnableAutoExit` (`HIDS hCam, INT nMode`) ⇝ `IDSEXP`
* [ ] `is_EnableMessage` (`HIDS hCam, INT which, HWND hWnd`) ⇝ `IDSEXP`
* [ ] `is_SetHardwareGain` (`HIDS hCam, INT nMaster, INT nRed, INT nGreen, INT nBlue`) ⇝ `IDSEXP`
* [ ] `is_SetWhiteBalance` (`HIDS hCam, INT nMode`) ⇝ `IDSEXP`
* [ ] `is_SetWhiteBalanceMultipliers` (`HIDS hCam, double dblRed, double dblGreen, double dblBlue`) ⇝ `IDSEXP`
* [ ] `is_GetWhiteBalanceMultipliers` (`HIDS hCam, double *pdblRed, double *pdblGreen, double *pdblBlue`) ⇝ `IDSEXP`
* [ ] `is_SetColorCorrection` (`HIDS hCam, INT nEnable, double *factors`) ⇝ `IDSEXP`
* [ ] `is_SetSubSampling` (`HIDS hCam, INT mode`) ⇝ `IDSEXP`
* [ ] `is_ForceTrigger` (`HIDS hCam`) ⇝ `IDSEXP`
* [ ] `is_GetBusSpeed` (`HIDS hCam`) ⇝ `IDSEXP`
* [ ] `is_SetBinning` (`HIDS hCam, INT mode`) ⇝ `IDSEXP`
* [ ] `is_ResetToDefault` (`HIDS hCam`) ⇝ `IDSEXP`
* [ ] `is_SetCameraID` (`HIDS hCam, INT nID`) ⇝ `IDSEXP`
* [ ] `is_SetBayerConversion` (`HIDS hCam, INT nMode`) ⇝ `IDSEXP`
* [ ] `is_SetHardwareGamma` (`HIDS hCam, INT nMode`) ⇝ `IDSEXP`
* [ ] `is_GetCameraList` (`PUEYE_CAMERA_LIST pucl`) ⇝ `IDSEXP`
* [ ] `is_SetAutoParameter` (`HIDS hCam, INT param, double *pval1, double *pval2`) ⇝ `IDSEXP`
* [ ] `is_GetAutoInfo` (`HIDS hCam, UEYE_AUTO_INFO *pInfo`) ⇝ `IDSEXP`
* [ ] `is_GetImageHistogram` (`HIDS hCam, int nMemId, INT ColorMode, DWORD* pHistoMem`) ⇝ `IDSEXP`
* [ ] `is_SetTriggerDelay` (`HIDS hCam, INT nTriggerDelay`) ⇝ `IDSEXP`
* [ ] `is_SetGainBoost` (`HIDS hCam, INT mode`) ⇝ `IDSEXP`
* [ ] `is_SetGlobalShutter` (`HIDS hCam, INT mode`) ⇝ `IDSEXPDEP`
* [ ] `is_SetExtendedRegister` (`HIDS hCam, INT index,WORD value`) ⇝ `IDSEXP`
* [ ] `is_GetExtendedRegister` (`HIDS hCam, INT index, WORD *pwValue`) ⇝ `IDSEXP`
* [ ] `is_SetHWGainFactor` (`HIDS hCam, INT nMode, INT nFactor`) ⇝ `IDSEXP`
* [ ] `is_Renumerate` (`HIDS hCam, INT nMode`) ⇝ `IDSEXP`
* [ ] `is_WriteI2C` (`HIDS hCam, INT nDeviceAddr, INT nRegisterAddr, BYTE* pbData, INT nLen`) ⇝ `IDSEXP`
* [ ] `is_ReadI2C` (`HIDS hCam, INT nDeviceAddr, INT nRegisterAddr, BYTE* pbData, INT nLen`) ⇝ `IDSEXP`
* [ ] `is_GetHdrMode` (`HIDS hCam, INT *Mode`) ⇝ `IDSEXP`
* [ ] `is_EnableHdr` (`HIDS hCam, INT Enable`) ⇝ `IDSEXP`
* [ ] `is_SetHdrKneepoints` (`HIDS hCam, KNEEPOINTARRAY *KneepointArray, INT KneepointArraySize`) ⇝ `IDSEXP`
* [ ] `is_GetHdrKneepoints` (`HIDS hCam, KNEEPOINTARRAY *KneepointArray, INT KneepointArraySize`) ⇝ `IDSEXP`
* [ ] `is_GetHdrKneepointInfo` (`HIDS hCam, KNEEPOINTINFO *KneepointInfo, INT KneepointInfoSize`) ⇝ `IDSEXP`
* [ ] `is_SetOptimalCameraTiming` (`HIDS hCam, INT Mode, INT Timeout, INT *pMaxPxlClk, double *pMaxFrameRate`) ⇝ `IDSEXP`
* [ ] `is_GetSupportedTestImages` (`HIDS hCam, INT *SupportedTestImages`) ⇝ `IDSEXP`
* [ ] `is_GetTestImageValueRange` (`HIDS hCam, INT TestImage, INT *TestImageValueMin, INT *TestImageValueMax`) ⇝ `IDSEXP`
* [ ] `is_SetSensorTestImage` (`HIDS hCam, INT Param1, INT Param2`) ⇝ `IDSEXP`
* [ ] `is_GetColorConverter` (`HIDS hCam, INT ColorMode, INT *pCurrentConvertMode, INT *pDefaultConvertMode, INT *pSupportedConvertModes`) ⇝ `IDSEXP`
* [ ] `is_SetColorConverter` (`HIDS hCam, INT ColorMode, INT ConvertMode`) ⇝ `IDSEXP`
* [ ] `is_ImageQueue` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParams`) ⇝ `IDSEXP`
* [ ] `is_WaitForNextImage` (`HIDS hCam, UINT timeout, char** ppcMem, INT* pnMemId`) ⇝ `IDSEXPDEP`
* [ ] `is_InitImageQueue` (`HIDS hCam, INT nMode`) ⇝ `IDSEXPDEP`
* [ ] `is_ExitImageQueue` (`HIDS hCam`) ⇝ `IDSEXPDEP`
* [ ] `is_SetTimeout` (`HIDS hCam, UINT nMode, UINT Timeout`) ⇝ `IDSEXP`
* [ ] `is_GetTimeout` (`HIDS hCam, UINT nMode, UINT *pTimeout`) ⇝ `IDSEXP`
* [ ] `is_GetDuration` (`HIDS hCam, UINT nMode, INT* pnTime`) ⇝ `IDSEXP`
* [ ] `is_GetSensorScalerInfo` (`HIDS hCam, SENSORSCALERINFO *pSensorScalerInfo, INT nSensorScalerInfoSize`) ⇝ `IDSEXP`
* [ ] `is_SetSensorScaler` (`HIDS hCam, UINT nMode, double dblFactor`) ⇝ `IDSEXP`
* [ ] `is_GetImageInfo` (`HIDS hCam, INT nMemId, UEYEIMAGEINFO *pImageInfo, INT nImageInfoSize`) ⇝ `IDSEXP`
* [ ] `is_ImageFormat` (`HIDS hCam, UINT nCommand, void *pParam, UINT nSizeOfParam`) ⇝ `IDSEXP`
* [ ] `is_FaceDetection` (`HIDS hCam, UINT nCommand, void *pParam, UINT nSizeOfParam`) ⇝ `IDSEXP`
* [ ] `is_Focus` (`HIDS hCam, UINT nCommand, void *pParam, UINT nSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_ImageStabilization` (`HIDS hCam, UINT nCommand, void *pParam, UINT nSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_ScenePreset` (`HIDS hCam, UINT nCommand, void *pParam, UINT nSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_Zoom` (`HIDS hCam, UINT nCommand, void *pParam, UINT nSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_Sharpness` (`HIDS hCam, UINT nCommand, void *pParam, UINT nSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_Saturation` (`HIDS hCam, UINT nCommand, void *pParam, UINT nSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_TriggerDebounce` (`HIDS hCam, UINT nCommand, void *pParam, UINT nSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_ColorTemperature` (`HIDS hCam, UINT nCommand, void *pParam, UINT nSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_DirectRenderer` (`HIDS hCam, UINT nMode, void *pParam, UINT SizeOfParam`) ⇝ `IDSEXP`
* [x] `is_HotPixel` (`HIDS hCam, UINT nMode, void *pParam, UINT SizeOfParam`) ⇝ `IDSEXP`
* [ ] `is_AOI` (`HIDS hCam, UINT nCommand, void *pParam, UINT SizeOfParam`) ⇝ `IDSEXP`
* [x] `is_Transfer` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_BootBoost` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_DeviceFeature` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_Exposure` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_Trigger` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_DeviceInfo` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_OptimalCameraTiming` (`HIDS hCam, UINT u32Command, void* pParam, UINT u32SizeOfParam`) ⇝ `IDSEXP`
* [x] `is_SetPacketFilter` (`INT iAdapterID, UINT uFilterSetting`) ⇝ `IDSEXP`
* [x] `is_GetComportNumber` (`HIDS hCam, UINT *pComportNumber`) ⇝ `IDSEXP`
* [x] `is_IpConfig` (`INT iID, UEYE_ETH_ADDR_MAC mac, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_Configuration` (`UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_IO` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_AutoParameter` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_Convert` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_ParameterSet` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_EdgeEnhancement` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_PixelClock` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_ImageFile` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_Blacklevel` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_ImageBuffer` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_Measure` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_LUT` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParams`) ⇝ `IDSEXP`
* [x] `is_Gamma` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParams`) ⇝ `IDSEXP`
* [x] `is_Memory` (`HIDS hf, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXPDEP`
* [x] `is_Multicast` (` HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParams `) ⇝ `IDSEXP`
* [x] `is_Sequencer` (` HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParams `) ⇝ `IDSEXP`
* [x] `is_PersistentMemory` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_PowerDelivery` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParams`) ⇝ `IDSEXP`
* [x] `is_Event` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [x] `is_CaptureConfiguration` (`HIDS hCam, UINT nCommand, void* pParam, UINT cbSizeOfParam`) ⇝ `IDSEXP`
* [ ] `is_SetVideoInput` (`HIDS hf, INT Source`) ⇝ `IDSEXP`
* [ ] `is_SetHue` (`HIDS hf, INT Hue`) ⇝ `IDSEXP`
* [ ] `is_SetVideoMode` (`HIDS hf, INT Mode`) ⇝ `IDSEXP`
* [ ] `is_SetSyncLevel` (`HIDS hf, INT Level`) ⇝ `IDSEXP`
* [x] `is_ShowColorBars` (`HIDS hf, INT Mode`) ⇝ `IDSEXP`
* [ ] `is_SetScaler` (`HIDS hf, float Scalex, float Scaley`) ⇝ `IDSEXP`
* [ ] `is_SetHorFilter` (`HIDS hf, INT Mode`) ⇝ `IDSEXP`
* [x] `is_SetVertFilter` (`HIDS hf, INT Mode`) ⇝ `IDSEXP`
* [x] `is_ScaleDDOverlay` (`HIDS hf, BOOL boScale`) ⇝ `IDSEXP`
* [ ] `is_GetCurrentField` (`HIDS hf, int* pField`) ⇝ `IDSEXP`
* [ ] `is_SetVideoSize` (`HIDS hf, INT xpos, INT ypos, INT xsize, INT ysize`) ⇝ `IDSEXP`
* [x] `is_SetKeyOffset` (`HIDS hf, INT nOffsetX, INT nOffsetY`) ⇝ `IDSEXP`
* [ ] `is_SetParentHwnd` (`HIDS hf, HWND hwnd`) ⇝ `IDSEXP`
* [ ] `is_SetUpdateMode` (`HIDS hf, INT mode`) ⇝ `IDSEXP`
* [x] `is_OvlSurfaceOffWhileMove` (`HIDS hf, BOOL boMode`) ⇝ `IDSEXP`
* [ ] `is_GetPciSlot` (`HIDS hf, INT* pnSlot`) ⇝ `IDSEXP`
* [ ] `is_GetIRQ` (`HIDS hf, INT* pnIRQ`) ⇝ `IDSEXP`
* [ ] `is_SetToggleMode` (`HIDS hf, int nInput1, int nInput2, int nInput3, int nInput4`) ⇝ `IDSEXP`
* [ ] `is_SetDecimationMode` (`HIDS hf, INT nMode, INT nDecimate`) ⇝ `IDSEXP`
* [ ] `is_SetSync` (`HIDS hf, INT nSync`) ⇝ `IDSEXP`
* [ ] `is_SetVideoCrossbar` (`HIDS hf, INT In, INT Out`) ⇝ `IDSEXP`
* [ ] `is_WatchdogTime` (`HIDS hf, long lTime`) ⇝ `IDSEXP`
* [ ] `is_Watchdog` (`HIDS hf, long lMode`) ⇝ `IDSEXP`
* [ ] `is_SetPassthrough` (`HIDS hf, INT Source`) ⇝ `IDSEXP`
* [ ] `is_SetAGC` (`HIDS hCam, INT Mode`) ⇝ `IDSEXP`
* [ ] `is_SetCaptureMode` (`HIDS hCam, INT Mode`) ⇝ `IDSEXP`
* [ ] `is_SetRenderMode` (`HIDS hCam, INT Mode`) ⇝ `IDSEXP`
* [ ] `is_InitBoard` (`HIDS* phf, HWND hWnd`) ⇝ `IDSEXP`
* [ ] `is_ExitBoard` (`HIDS hf`) ⇝ `IDSEXP`
* [ ] `is_InitFalcon` (`HIDS* phf, HWND hWnd`) ⇝ `IDSEXP`
* [ ] `is_ExitFalcon` (`HIDS hf`) ⇝ `IDSEXP`
* [ ] `is_GetBoardType` (`HIDS hf`) ⇝ `IDSEXP`
* [ ] `is_GetBoardInfo` (`HIDS hf, PBOARDINFO pInfo`) ⇝ `IDSEXP`
* [ ] `is_BoardStatus` (`HIDS hf, INT nInfo, ULONG ulValue`) ⇝ `IDSEXPUL`
* [ ] `is_GetNumberOfBoards` (`INT* pnNumBoards`) ⇝ `IDSEXP`
* [ ] `is_SetBrightness` (`HIDS hf, INT Bright`) ⇝ `IDSEXP`
* [ ] `is_SetContrast` (`HIDS hf, INT Cont`) ⇝ `IDSEXP`
* [x] `is_GetDC` (`HIDS hf, HDC* phDC`) ⇝ `IDSEXP`
* [x] `is_ReleaseDC` (`HIDS hf, HDC hDC`) ⇝ `IDSEXP`
* [x] `is_UpdateDisplay` (`HIDS hf`) ⇝ `IDSEXP`
* [x] `is_SetDisplaySize` (`HIDS hf, INT x, INT y`) ⇝ `IDSEXP`
* [x] `is_LockDDOverlayMem` (`HIDS hf, VOID** ppMem, INT* pPitch`) ⇝ `IDSEXP`
* [x] `is_UnlockDDOverlayMem` (`HIDS hf`) ⇝ `IDSEXP`
* [x] `is_LockDDMem` (`HIDS hf, VOID** ppMem, INT* pPitch`) ⇝ `IDSEXP`
* [x] `is_UnlockDDMem` (`HIDS hf`) ⇝ `IDSEXP`
* [x] `is_GetDDOvlSurface` (`HIDS hf, void** ppDDSurf`) ⇝ `IDSEXP`
* [x] `is_SetKeyColor` (`HIDS hf, INT r, INT g, INT b`) ⇝ `IDSEXP`
* [x] `is_StealVideo` (`HIDS hf, int Wait`) ⇝ `IDSEXP`
* [x] `is_SetDDUpdateTime` (`HIDS hf, INT ms`) ⇝ `IDSEXP`
* [x] `is_EnableDDOverlay` (`HIDS hf`) ⇝ `IDSEXP`
* [ ] `is_DisableDDOverlay` (`HIDS hf`) ⇝ `IDSEXP`
* [x] `is_ShowDDOverlay` (`HIDS hf`) ⇝ `IDSEXP`
* [x] `is_HideDDOverlay` (`HIDS hf`) ⇝ `IDSEXP`
* [ ] `is_GetExposureRange` (`HIDS hf, double *min, double *max, double *intervall`) ⇝ `IDSEXP`
* [ ] `is_SetExposureTime` (`HIDS hf, double EXP, double* newEXP`) ⇝ `IDSEXP`
* [ ] `is_SetBadPixelCorrection` (`HIDS hf, INT nEnable, INT threshold`) ⇝ `IDSEXP`
* [ ] `is_LoadBadPixelCorrectionTable` (`HIDS hf, const IS_CHAR *File`) ⇝ `IDSEXP`
* [ ] `is_SaveBadPixelCorrectionTable` (`HIDS hf, const IS_CHAR *File`) ⇝ `IDSEXP`
* [ ] `is_SetBadPixelCorrectionTable` (`HIDS hf, INT nMode, WORD *pList`) ⇝ `IDSEXP`
* [ ] `is_SetMemoryMode` (`HIDS hf, INT nCount, INT nDelay`) ⇝ `IDSEXP`
* [ ] `is_TransferImage` (`HIDS hf, INT nMemID, INT seqID, INT imageNr, INT reserved`) ⇝ `IDSEXP`
* [ ] `is_TransferMemorySequence` (`HIDS hf, INT seqID, INT StartNr, INT nCount, INT nSeqPos`) ⇝ `IDSEXP`
* [ ] `is_MemoryFreezeVideo` (`HIDS hf, INT nMemID, INT Wait`) ⇝ `IDSEXP`
* [ ] `is_GetLastMemorySequence` (`HIDS hf, INT *pID`) ⇝ `IDSEXP`
* [ ] `is_GetNumberOfMemoryImages` (`HIDS hf, INT nID, INT *pnCount`) ⇝ `IDSEXP`
* [ ] `is_GetMemorySequenceWindow` (`HIDS hf, INT nID, INT *left, INT *top, INT *right, INT *bottom`) ⇝ `IDSEXP`
* [ ] `is_IsMemoryBoardConnected` (`HIDS hf, BOOL *pConnected`) ⇝ `IDSEXP`
* [ ] `is_ResetMemory` (`HIDS hf, INT nReserved`) ⇝ `IDSEXP`
* [ ] `is_SetTestImage` (`HIDS hf, INT nMode`) ⇝ `IDSEXP`
* [ ] `is_GetCaptureErrorInfo` (`HIDS hf, UEYE_CAPTURE_ERROR_INFO *pCaptureErrorInfo, UINT SizeCaptureErrorInfo`) ⇝ `IDSEXP`
* [ ] `is_ResetCaptureErrorInfo` (`HIDS hf `) ⇝ `IDSEXP`
* [ ] `is_SetAutoCfgIpSetup` (`INT iAdapterID, const UEYE_ETH_AUTOCFG_IP_SETUP* pSetup, UINT uStructSize`) ⇝ `IDSEXP`
* [ ] `is_SetPersistentIpCfg` (`HIDS hf, UEYE_ETH_IP_CONFIGURATION* pIpCfg, UINT uStructSize`) ⇝ `IDSEXP`
* [ ] `is_ConvertImage` (`HIDS hf, char* pcSource, int nIDSource, char** pcDest, INT *nIDDest, INT *reserved`) ⇝ `IDSEXP`
* [ ] `is_SetConvertParam` (`HIDS hf, BOOL ColorCorrection, INT BayerConversionMode, INT ColorMode, INT Gamma, double* WhiteBalanceMultipliers`) ⇝ `IDSEXP`
* [ ] `is_SetEdgeEnhancement` (`HIDS hf, INT nEnable`) ⇝ `IDSEXP`
* [ ] `is_SetBlCompensation` (`HIDS hCam, INT nEnable, INT offset, INT reserved`) ⇝ `IDSEXP`
* [ ] `is_LoadParameters` (`HIDS hf, const IS_CHAR* pFilename`) ⇝ `IDSEXP`
* [ ] `is_SaveParameters` (`HIDS hf, const IS_CHAR* pFilename`) ⇝ `IDSEXP`
* [ ] `is_GetPixelClockRange` (`HIDS hf, INT *pnMin, INT *pnMax`) ⇝ `IDSEXP`
* [ ] `is_SetPixelClock` (`HIDS hf, INT Clock`) ⇝ `IDSEXP`
* [ ] `is_GetOsVersion` (`void`) ⇝ `IDSEXP`
* [ ] `is_SetImageSize` (`HIDS hf, INT x, INT y`) ⇝ `IDSEXP`
* [ ] `is_SetImagePos` (`HIDS hf, INT x, INT y`) ⇝ `IDSEXP`
* [ ] `is_SetAOI` (`HIDS hf, INT cmd, INT *pXPos, INT *pYPos, INT *pWidth, INT *pHeight`) ⇝ `IDSEXP`
* [ ] `is_SetImageAOI` (`HIDS hf, INT xPos, INT yPos, INT width, INT height`) ⇝ `IDSEXP`
* [ ] `is_SetIO` (`HIDS hf, INT nIO`) ⇝ `IDSEXP`
* [ ] `is_SetIOMask` (`HIDS hf, INT nMask`) ⇝ `IDSEXP`
* [ ] `is_SetLED` (`HIDS hf, INT nValue`) ⇝ `IDSEXP`
* [ ] `is_GetGlobalFlashDelays` (`HIDS hf, ULONG *pulDelay, ULONG *pulDuration`) ⇝ `IDSEXP`
* [ ] `is_SetFlashDelay` (`HIDS hf, ULONG ulDelay, ULONG ulDuration`) ⇝ `IDSEXP`
* [ ] `is_SetFlashStrobe` (`HIDS hf, INT nMode, INT nLine`) ⇝ `IDSEXP`
* [ ] `is_GetEthDeviceInfo` (`HIDS hf, UEYE_ETH_DEVICE_INFO* pDeviceInfo, UINT uStructSize`) ⇝ `IDSEXP`
* [ ] `is_LoadImage` (`HIDS hf, const IS_CHAR* File`) ⇝ `IDSEXP`
* [ ] `is_LoadImageMem` (`HIDS hf, const IS_CHAR* File, char** ppcImgMem, INT* pid`) ⇝ `IDSEXP`
* [ ] `is_SaveImage` (`HIDS hf, const IS_CHAR* File`) ⇝ `IDSEXP`
* [ ] `is_SaveImageMem` (`HIDS hf, const IS_CHAR* File, char* pcMem, int nID`) ⇝ `IDSEXP`
* [ ] `is_SaveImageEx` (`HIDS hf, const IS_CHAR* File, INT fileFormat, INT Param`) ⇝ `IDSEXP`
* [ ] `is_SaveImageMemEx` (`HIDS hf, const IS_CHAR* File, char* pcMem, INT nID, INT FileFormat, INT Param`) ⇝ `IDSEXP`
* [ ] `is_SetCameraLUT` (`HIDS hCam, UINT Mode, UINT NumberOfEntries, double *pRed_Grey, double *pGreen, double *pBlue`) ⇝ `IDSEXP`
* [ ] `is_GetCameraLUT` (`HIDS hCam, UINT Mode, UINT NumberOfEntries, double *pRed_Grey, double *pGreen, double *pBlue`) ⇝ `IDSEXP`
* [ ] `is_SetGamma` (`HIDS hCam, INT nGamma`) ⇝ `IDSEXP`
* [x] `is_ReadEEPROM` (`HIDS hCam, INT Adr, char* pcString, INT Count`) ⇝ `IDSEXP`
* [x] `is_WriteEEPROM` (`HIDS hCam, INT Adr, char* pcString, INT Count`) ⇝ `IDSEXP`
