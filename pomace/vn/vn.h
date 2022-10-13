//
//  vn.h
//  vn
//
//  Created by Yury Korolev on 13.10.2022.
//


#import <Foundation/Foundation.h>
#import <Vision/Vision.h>
#import "../macro.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - VNRequest

#pragma mark - VNImageRequestHandler

NS_RETURNS_RETAINED
cinit_ab(, VNImageRequestHandler, initWithCVPixelBuffer, CVPixelBufferRef, options, NSDictionary *);


NS_RETURNS_RETAINED
cinit_abc(, VNImageRequestHandler, initWithCVPixelBuffer, CVPixelBufferRef, orientation, CGImagePropertyOrientation, options, NSDictionary *);

rsel_ab(, id, performRequests, NSArray<VNRequest *> *, error, NSError **, BOOL)

#pragma mark - VNSequenceRequestHandler

csel(, VNSequenceRequestHandler, new, VNSequenceRequestHandler *)

//- (BOOL)performRequests:(NSArray<VNRequest *> *)requests onCVPixelBuffer:(CVPixelBufferRef)pixelBuffer error:(NSError **)error;

rsel_abc(, id, performRequests, NSArray<VNRequest *> *, onCVPixelBuffer, CVPixelBufferRef, error, NSError **, BOOL)


NS_ASSUME_NONNULL_END

