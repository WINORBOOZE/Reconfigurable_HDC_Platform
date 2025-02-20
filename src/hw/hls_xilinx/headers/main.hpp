/**
 * @file main.hpp
 *
 * @author Marco Angioli and Saeid Jamili
 * @email marco.angioli@uniroma1.it and saeid.jamili@uniroma1.it
 * @note Author names are listed in alphabetical order.
 * @date Created on: 12th August 2023
 * @date Last updated on: 5th May 2024
 * @institution Sapienza University of Rome
 *
 * @section LICENSE
    Copyright 2024 Sapienza University of Rome

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
    Authors: Marco Angioli & Saeid Jamili

 * @section CHANGELOG
 * @version 1.0.2
 * @date May 2024
 */

#ifndef MAIN_H
#define MAIN_H


// Standard library includes
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

// Project-specific includes
#include "ap_int.h"
#include "datatypes.hpp"

// Third-party libraries
#include <hls_stream.h>
#include "ap_axi_sdata.h"
#include "../lib/hdc_lib/HDC_class.hpp"


// Main HDV Engine function prototype
void hdv_engine(

	 bool &nrst_i,
		op_mode_t op_mode_i,
		frame_in_t frame_in,
		#if IN_DATA_MODE == DI_M_STREAM
			AXI_STREAM& sdata_i,
		#elif ( IN_DATA_MODE == DI_M_PARTIAL_PARALLEL || IN_DATA_MODE == DI_M_PARALLEL )
			dataf_in_p_t &df_i,
		#endif

		#if ( CV_MODE  == CV_M_INT_MEM )
			chv_p_t &chv_i,
			#if ( TRAIN_ON_HW )
				chv_t *chv_o,
			#endif
		#endif
		#if ( BV_MODE  == BV_M_INT_MEM )
			bhv_p_t &bhv_i,
		#endif
		#if ( LV_MODE == LV_M_INT_MEM )
			lhv_p_t &lhv_i,
		#endif
		#if ( LV_MODE == LV_M_LOGIC &&  HD_LV_TYPE == APPROX )
			lhv_p_t &lhv_i0,
			lhv_p_t &lhv_i1,
		#endif

		#if ( AXI_CNTR_PORT_EN )
			if_axi_cmd_t &axi_if_cmd,
			if_axi_data_t *axi_if_data,
		#endif

		#if ( TRAIN_ON_HW )
			pred_class_t &lable_class_i,
		#endif

		#if ( LR_DECAY == LR_ITER )
			uint16_t &lr_in,
		#endif
		
		#if ( ENCODING_TECHNIQUE == ENCODING_NGRAM || N_GRAM == 1 )
			bool &permutation_repeat,
		#endif
		
		#if ( N_GRAM == 1 )
			uint8_t &n_gram_idx,
		#endif
		#if ( PROBLEM_TYPE != PROBLEM_TYPE_REGRESSION)
			pred_class_t *pred_class_o,
		#else
			int32_t *pred_value_o,
		#endif
		sys_status_t *status_o



	);


#endif
