//
//  ns.h
//  ns
//
//  Created by Yury Korolev on 07.07.2022.
//

#import <Foundation/Foundation.h>
#include "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - NSProcessInfo

NS_RETURNS_NOT_RETAINED
csel(, NSProcessInfo, processInfo, NSProcessInfo *)

rsel(, id, thermalState, NSProcessInfoThermalState)

//@property (readonly, getter=isLowPowerModeEnabled) BOOL lowPowerModeEnabled
rsel(, id, isLowPowerModeEnabled, BOOL)


NS_ASSUME_NONNULL_END
