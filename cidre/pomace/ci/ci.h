//
//  ci.h
//  ci
//
//  Created by Yury Korolev on 22.05.2022.
//

#import <Foundation/Foundation.h>
#import <CoreImage/CoreImage.h>
#import "../macro.h"

NS_ASSUME_NONNULL_BEGIN

Class CI_IMAGE;
Class CI_CONTEXT;

__attribute__((constructor))
static void common_initializer()
{
  static int initialized = 0;
  if (!initialized) {
    CI_IMAGE = [CIImage class];
    CI_CONTEXT = [CIContext class];
  }
}

NS_ASSUME_NONNULL_END

