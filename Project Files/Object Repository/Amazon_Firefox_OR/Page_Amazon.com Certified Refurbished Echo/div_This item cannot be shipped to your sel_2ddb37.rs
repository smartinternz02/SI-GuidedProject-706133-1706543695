<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_This item cannot be shipped to your sel_2ddb37</name>
   <tag></tag>
   <elementGuidId>ad1cc476-82fb-4c63-b666-f9facacb846c</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='desktop_qualifiedBuyBox']/div</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>#desktop_qualifiedBuyBox > div.a-section.a-spacing-none.a-padding-none</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>c1c712ae-77e8-4096-a6d3-151e9900894d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>a-section a-spacing-none a-padding-none</value>
      <webElementGuid>30426baa-9c66-46d4-a437-54f802379430</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>                               
                                                       
                                  
                                                    
                                  
                                                                 
                                                      
                            
                                                    
                            
                                                
                            
                                                                               
                            
                                 
                             
                            
                                                      
                            
                                                                                                                               
                            
                                                                 
                            
                                                                 
                            
                                                   
                                    
  


This item cannot be shipped to your selected delivery location. Please choose a different delivery location.

               
  
    
    P.when(&quot;A&quot;, &quot;jQuery&quot;).execute(function(A, $) {
        $(&quot;#selectQuantity [name='quantity'], #mobileQuantityDropDown&quot;).live(&quot;change&quot;, function (event) {

            if (event.updateDeliveryBlockOnQuantityChange) {
                return;
            }
            event.updateDeliveryBlockOnQuantityChange = 1;

            // &quot;#buybox&quot; is included in this list because if there is no accordion row, then it is a single-offer layout
            // possible id's may include &quot;usedAccordionRow&quot;, &quot;newAccordionRow_1&quot;, &quot;newAccordionRow_2&quot;
            var accordionRow = $(this).closest('[id$=&quot;AccordionRow&quot;], #buybox, [id^=&quot;newAccordionRow&quot;]');

            var quantity = $(this).val();
            var asin = accordionRow.find(&quot;#deliveryBlockSelectAsin&quot;).val();
            var merchantId = accordionRow.find(&quot;#deliveryBlockSelectMerchant&quot;).val();

            if (!asin || !merchantId) {
                return;
            }

            var params = [];

            params.push(&quot;asin=&quot; + asin);
            params.push(&quot;quantity=&quot; + quantity);
            params.push(&quot;exclusiveMerchantId=&quot; + merchantId);
            params.push(&quot;merchantId=&quot; + merchantId);
            params.push(&quot;clientId=retailwebsite&quot;);
            params.push(&quot;deviceType=web&quot;);
            params.push(&quot;showFeatures=deliveryBlock&quot;);
            params.push(&quot;ie=UTF8&quot;);
            params.push(&quot;experienceId=deliveryBlockQuantityRefreshAjaxExperience&quot;);

            // Weblab gated addition of Locale and OfferListingId to QuantityRefresh request
            var addLParamsToQuantityRefreshWeblabFlag = false;
            if (addLParamsToQuantityRefreshWeblabFlag) {
                var locale = accordionRow.find(&quot;#deliveryBlockSelectLocale&quot;).val();
                var offerListingId = accordionRow.find(&quot;#deliveryBlockSelectOfferListingId&quot;).val();

                // Only add language param if locale is non-null
                if (locale) {
                    params.push(&quot;language=&quot; + locale);
                }
                // Only add offerListingId param if value is non-null
                if (offerListingId) {
                    params.push(&quot;offerListingId=&quot; + offerListingId);
                }
            }

            // Weblab gated omboarding of MOD to UDM
            var onboardModToUdmWwWeblabTreatmentWeblabFlag = false;

            if (onboardModToUdmWwWeblabTreatmentWeblabFlag) {
                $.ajax({
                   type: &quot;GET&quot;,
                   url: &quot;/gp/product/ajax?&quot;,
                   contentType: 'application/x-www-form-urlencoded;charset=utf-8',
                   data: params.join('&amp;'),
                   accordionRow: accordionRow,
                   dataType: &quot;html&quot;,
                   success: function(objResponse) {
                    // add weblab gating?
                       if (objResponse != null &amp;&amp; objResponse != &quot;&quot;) {
                            // parse objResponse and extract it into DB Small and DB Large

                             // The specific string to split the HTML content
                             // Split the HTML content using the separator
                             var htmlContentArray = objResponse.split(&quot;##split##&quot;);

                             var objResponseDbLarge = htmlContentArray[0];
                             var objResponseDbSmall = htmlContentArray[1];

                             // null checks
                             if (objResponseDbLarge != null &amp;&amp; objResponseDbLarge != &quot;&quot;){
                                 accordionRow.find(&quot;#deliveryBlockMessage&quot;).replaceWith(objResponseDbLarge);
                             }
                             if (objResponseDbSmall != null &amp;&amp; objResponseDbSmall != &quot;&quot;){
                                 accordionRow.find(&quot;#deliveryBlockSmallMessage&quot;).replaceWith(objResponseDbSmall);
                             }
                       }
                   }
                });
            } else {
                $.ajax({
                    type: &quot;GET&quot;,
                    url: &quot;/gp/product/ajax?&quot;,
                    contentType: 'application/x-www-form-urlencoded;charset=utf-8',
                    data: params.join('&amp;'),
                    accordionRow: accordionRow,
                    dataType: &quot;html&quot;,
                    success: function(objResponse) {
                        if (objResponse != null &amp;&amp; objResponse != &quot;&quot;) {
                            accordionRow.find(&quot;#deliveryBlockMessage&quot;).replaceWith(objResponse);
                        }
                    }
               });
            }
            return;
        });
    });
                             
                                  
                                    
                                
                                
                                    Deliver to India
                                
                            
                                                          
                               
                            
                                                                        
                            
                                                                                       
                            
                                                                                                                      
                            
                           (function(f) {var _np=(window.P._namespace(&quot;promoRenameBuyBoxCXCW&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
    P.when('A', 'jQuery').execute(function(A, $) {
        $('#desktop_buybox').find('#promoPriceBlockMessage_feature_div').prop(&quot;id&quot;,&quot;promoPriceBlockMessageInBuyBox_feature_div&quot;);
    });
}));                           
                             
                                  
                                                 
                                  
                                                   
                                  
                                                                                                              Only 20 left in stock - order soon.                                                            
    .availabilityMoreDetailsIcon {
        width: 12px;
        vertical-align: baseline;
        fill: #969696;
    }
                              
                                  
                                                                               
                                  
                                                     
                                  
                                                                       
                                  
                                                    
    /* Adding this CSS to overridden the green badging styles */
    #bmsmMessaging span.a-text-bold {
        font-weight: normal !important;
        background-color: #7fda69;
        padding: 2px 6px;
    }

    #bmsmMessaging {
        font-weight: normal !important;
        margin-bottom: 12px !important;
        text-align: left;
        display:  none;
    }

    /* Only display quantity discount on qualified buy box; excluding pickup and other buy box */
    #qualifiedBuybox #bmsmMessaging {
        display: block !important;
    }

    #bmsmMessaging .a-icon.a-icon-popover {
        margin-top: 6px !important;
    }

    #bmsmMessaging .a-color-success {
        color: black !important;
    }

    @media (min-width: 801px){
        #bmsmMessaging span{
            font-size: 14px !important;
        }
    }

    /* mobile screen */
    @media (max-width: 800px){
        #bmsmMessaging span{
            font-size: 15px !important;
        }

        #bmsmMessaging span.a-text-bold {
            margin-right: 5px;
        }
    }


                                                                                                                      
			     Quantity:     1        2        3        4        5        6        7+     Quantity:1                      
        P.when('quantity-restriction-messaging').execute(function(quantityRestrictionMessaging){
            quantityRestrictionMessaging.quantityModal({&quot;popoverTitle&quot;:&quot;Your Device Order&quot;,&quot;moq&quot;:7,&quot;message&quot;:&quot;&lt;div data-csa-c-type=widget data-csa-c-slot-id=qrm-div data-csa-c-content-id=qrm-devices>Standard Amazon accounts are limited to fewer than MAX_QUANTITY_PLUS_ONE units.  Larger quantities may be ordered with an &lt;a href=\&quot;/b/?ie=UTF8&amp;node=23544440011\&quot;>Amazon Business&lt;\/a> account. Sign in with or create an Amazon Business account to continue. Amazon does not offer retail promotional discounts for bulk device orders and all bundle offers involving Amazon devices are limited to one per customer.&lt;\/div>&quot;,&quot;cspMessage&quot;:&quot;To discuss options for quantities higher than MAX_QUANTITY_PLUS_ONE units, please &lt;a href=\&quot;mailto:amazon-bulk-device-sales@amazon.com?subject=Amazon%20Bulk%20Device%20Request%20-%20Negotiated%20price&amp;body=Amazon%20is%20looking%20forward%20to%20supporting%20your%20negotiated%20price%20device%20request.%0d%0dPlease%20provide%20the%20following%20information%20so%20your%20account%20manager%20can%20better%20assist%20you. %0d%0dFirst%20and%20Last%20Name:%20%0dPhone%20Number:%20%0dBusiness%20Name:%20%0dWhich%20device(s)%20are%20you%20seeking?%20%0dWhat%20quantity%20are%20you%20seeking?%20%0dWho%20and%20where%20are%20the%20end%20customers?%20%0dWhen%20do%20you%20need%20devices%20in%20hand?%20%0d\&quot;>email your account manager.&lt;\/a> &quot;,&quot;cspMOQ&quot;:&quot;1000&quot;})
        });
     
                                             
                                  
                                                            
                                  
                                      
    
    

           $$48.9948.99  
                    ()
              Includes selected options.   Includes initial monthly payment and selected options.          
                         Details  
                          Price     ($48.99x)          $48.99          Subtotal     $$48.9948.99      Subtotal                  Initial payment breakdown           Shipping cost, delivery date, and order total (including tax) shown at checkout. 
                                                    
                                  
                                                     
                                  
                                                         
                                  
                           
             Similar items shipping to India
              IN
                India
             
        
         if(window.mix_csa){window.mix_csa('[cel_widget_id=&quot;export-alternative-card-desktop_desktop-export-alternative_0&quot;]', '#CardInstancemvDQsFXTLA2hZ3IKqUIS9Q')('mark', 'bb')}
if(window.uet){window.uet('bb','export-alternative-card-desktop_desktop-export-alternative_0',{wb: 1})}
._export-alternative-card-desktop_style_export_alternative_table_desktop__2ehnZ{background:none!important;background-color:none!important;width:99%}._export-alternative-card-desktop_style_export_alternative_table_first_empty_col__3xZRw,._export-alternative-card-desktop_style_export_alternative_table_mobile__3hpwj{background:none!important;background-color:none!important}._export-alternative-card-desktop_style_export_alternative_table_first_empty_col__3xZRw{width:160px}._export-alternative-card-desktop_style_export_alternative_table_first_label_col__ivoXZ{background-color:#e0e0e0!important;padding-top:5px!important;width:160px}._export-alternative-card-desktop_style_export_alternative_table_image_title_row__31u-4{background:none!important;background-color:none!important;border-left-style:hidden;border-right-style:hidden;border-top-style:hidden}._export-alternative-card-desktop_style_export_alternative_table_image_title_cell__2L2zL{background:none!important;background-color:none!important;border-style:hidden}._export-alternative-card-desktop_style_export_alternative_table_cta_row__3lFCu{background:none!important;background-color:none!important;border-bottom-style:hidden;border-left-style:hidden;border-right-style:hidden}._export-alternative-card-desktop_style_export_alternative_table_cta_cell__1c3f_{background:none!important;background-color:none!important;border-style:hidden}._export-alternative-card-desktop_style_export_alternative_energy_efficiency_cell__2667h{padding-right:5px!important;padding-top:10px!important}._export-alternative-card-desktop_style_export_alternative_in_cell_postion__1M5sG{padding-right:15px!important;padding-top:5px!important}._export-alternative-card-desktop_style_export_alternative_shipping_fee_row__C-mis{height:120px!important}
._export-alternative-card-desktop_style_export_alternative_cta_button__22AJK{width:100%}._export-alternative-card-desktop_style_export_alternative_image_shift__rrAwZ{padding-left:25%!important}._export-alternative-card-desktop_style_export_alternative_energy_efficiency_desktop__2zh3v{background:none!important;background-color:none!important;margin:0;overflow:hidden;padding:0}._export-alternative-card-desktop_style_export_alternative_energy_efficiency_mobile__3Z6NM{background:none!important;background-color:none!important;margin:0;overflow:hidden;padding:0;position:relative}
._export-alternative-card-desktop_energy-efficiency_energy-efficiency-container__1Pkva{text-align:left}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-standard__28gp8{cursor:pointer;display:inline-block;height:24px}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-shape__1IcJY{display:inline-block;height:24px}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-rating__3_0eN{fill:#fff;font-size:20px;vertical-align:middle}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK{fill:#fff;font-size:14px;vertical-align:middle}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P{left:24px * .6;text-shadow:-.5px -.5px 0 #000,.5px -.5px 0 #000,-.5px .5px 0 #000,.5px .5px 0 #000}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2{display:inline-block;padding-left:5px;padding-top:0;position:absolute;vertical-align:middle}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3{cursor:pointer;word-break:break-word}
._export-alternative-card-desktop_style_export_alternative_trigger_button__3e7gX{width:100%}
See Similar Itemsif(window.mix_csa){window.mix_csa('[cel_widget_id=&quot;export-alternative-card-desktop_desktop-export-alternative_0&quot;]', '#CardInstancemvDQsFXTLA2hZ3IKqUIS9Q')('mark', 'be')}
if(window.uet){window.uet('be','export-alternative-card-desktop_desktop-export-alternative_0',{wb: 1})}
if(window.mixTimeout){window.mixTimeout('export-alternative-card-desktop', 'CardInstancemvDQsFXTLA2hZ3IKqUIS9Q', 90000)};
P.when('mix:@amzn/mix.client-runtime', 'mix:export-alternative-card-desktop__OD63ZHOz').execute(function (runtime, cardModule) {runtime.registerCardFactory('CardInstancemvDQsFXTLA2hZ3IKqUIS9Q', cardModule).then(function(){if(window.mix_csa){window.mix_csa('[cel_widget_id=&quot;export-alternative-card-desktop_desktop-export-alternative_0&quot;]', '#CardInstancemvDQsFXTLA2hZ3IKqUIS9Q')('mark', 'functional')}if(window.uex){window.uex('ld','export-alternative-card-desktop_desktop-export-alternative_0',{wb: 1})}});});

P.load.js('https://images-na.ssl-images-amazon.com/images/I/31PCZfmrW-L.js?xcp');

                             
                                  
                                                                                                                                          {&quot;shouldUseNatc&quot;:true}                                    
                             Add to Cart            
  


This item cannot be shipped to your selected delivery location. Please choose a different delivery location.

       
                                                 
                                  
                                                                            
                                  
                                 Enhancements you chose aren't available for this seller.          Details         To add the following enhancements to your purchase, choose a different seller.     %cardName%         ${cardName} not available for the seller you chose     ${cardName} unavailable for quantities greater than ${maxQuantity}.                                
                                  
                                                    
                                  
                                                               
                                                         
                                                        
                                  
                                 
    
        Ships from 


           
    Amazon.com         
       Ships from             Amazon.com                                      
                                  
                                 
    
        Sold by 


           
    Amazon.com         
       Sold by             Amazon.com                                      
                                  
                                 
    
        Returns 


                  Eligible for Return, Refund or Replacement within 30 days of receipt        Eligible for Return, Refund or Replacement within 30 days of receipt    This item can be returned in its original condition for a full refund or replacement within 30 days of receipt.      Read full return policy              
       Returns                Eligible for Return, Refund or Replacement within 30 days of receipt    This item can be returned in its original condition for a full refund or replacement within 30 days of receipt.      Read full return policy                                          
                                  
                                                        
                                  
                                                     
                                  
                                                     
                                  
                                                     
                                  
                                                      
                                  
                                                     
                                  
                                                     
                                  
                                                        
                                  
                              
                                    
                                                       
                                     
                                                       
                                     
                                                       
                                     
                                                       
                                     
                                       
    
        Payment 


                  Secure transaction        Your transaction is secure    We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.   Learn more               
       Payment                Secure transaction    We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.   Learn more                                            
                                     
                                                        
                                     
                                                       
         
                      
          
                                    Details      See more            
               
                                  
                                  
                                                                     
                                  
                                                                       
                                  
                                                                          
                                  
                                                      
                                  
                                                    
                                  
                                                    
                                  
                                                                     
                                  
                                                      
                                  
                                                     
                                  
                                                       
                                  
                                                       
                                  
                                                    
                                  
                                                          
                                  
                                                     
                                  
                                                     
                                  
                                                       
                                  
                                                     
                                  
                                                     
                                  
                                                    
                                  
                          {&quot;heroName&quot;:&quot;&quot;} {}                             
                                  
                                                    
                                  
                                                     
                                  
                                                    
                                  
                                                                                      
                                  
                               This is a gift                                   
                                  
                         
 
        Link device to your Amazon account   to simplify setup.        Why is this important?      Link this device to your Amazon account to enable Frustration-Free Setup.   If you already have a qualifying Amazon device connected to your home network and stored network credentials, the new device can automatically join the same home network and automatically complete device setup.   Learn more about Amazon Frustration-Free Setup         You’ve marked this as a gift, so it will not be linked to your account. To link to your account, first de-select &quot;This is a gift&quot;   Account linking is not available when ordering more than 3 of this device.     (function(f) {var _np=(window.P._namespace(&quot;PreRegistration&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
        P.when(&quot;PreRegistration.Controller&quot;, &quot;ready&quot;).execute(function(PreRegistrationController) {
        var controllerDataJson = {&quot;maxQuantity&quot;:3,&quot;defaultValue&quot;:true,&quot;showStatus&quot;:true,&quot;asin&quot;:&quot;B085M5XQVR&quot;,&quot;status&quot;:&quot;ALLOWED_CONDITIONAL&quot;};
        PreRegistrationController.getInstance(controllerDataJson);
        });
    }));                        
                                  
                                                 
                                  
                                                      
                                  
                                                     
          </value>
      <webElementGuid>6ce26b2c-c340-405d-8b0f-321a7327c9f6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;desktop_qualifiedBuyBox&quot;)/div[@class=&quot;a-section a-spacing-none a-padding-none&quot;]</value>
      <webElementGuid>172172e8-b0a1-43a2-b3fa-634685f52be2</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='desktop_qualifiedBuyBox']/div</value>
      <webElementGuid>ac570626-dad3-4586-afd5-7b5242fa9851</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='$48.99'])[1]/following::div[3]</value>
      <webElementGuid>243ed22a-5fed-46b7-bddd-c112fd7242ff</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Trade-In'])[1]/following::div[56]</value>
      <webElementGuid>78c69ca9-51a5-4166-b2bb-9aeff657bf02</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div/div/div[4]/div</value>
      <webElementGuid>8b78bd84-f563-4e1b-9821-5e400f29027f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;                               
                                                       
                                  
                                                    
                                  
                                                                 
                                                      
                            
                                                    
                            
                                                
                            
                                                                               
                            
                                 
                             
                            
                                                      
                            
                                                                                                                               
                            
                                                                 
                            
                                                                 
                            
                                                   
                                    
  


This item cannot be shipped to your selected delivery location. Please choose a different delivery location.

               
  
    
    P.when(&quot;A&quot;, &quot;jQuery&quot;).execute(function(A, $) {
        $(&quot;#selectQuantity [name=&quot; , &quot;'&quot; , &quot;quantity&quot; , &quot;'&quot; , &quot;], #mobileQuantityDropDown&quot;).live(&quot;change&quot;, function (event) {

            if (event.updateDeliveryBlockOnQuantityChange) {
                return;
            }
            event.updateDeliveryBlockOnQuantityChange = 1;

            // &quot;#buybox&quot; is included in this list because if there is no accordion row, then it is a single-offer layout
            // possible id&quot; , &quot;'&quot; , &quot;s may include &quot;usedAccordionRow&quot;, &quot;newAccordionRow_1&quot;, &quot;newAccordionRow_2&quot;
            var accordionRow = $(this).closest(&quot; , &quot;'&quot; , &quot;[id$=&quot;AccordionRow&quot;], #buybox, [id^=&quot;newAccordionRow&quot;]&quot; , &quot;'&quot; , &quot;);

            var quantity = $(this).val();
            var asin = accordionRow.find(&quot;#deliveryBlockSelectAsin&quot;).val();
            var merchantId = accordionRow.find(&quot;#deliveryBlockSelectMerchant&quot;).val();

            if (!asin || !merchantId) {
                return;
            }

            var params = [];

            params.push(&quot;asin=&quot; + asin);
            params.push(&quot;quantity=&quot; + quantity);
            params.push(&quot;exclusiveMerchantId=&quot; + merchantId);
            params.push(&quot;merchantId=&quot; + merchantId);
            params.push(&quot;clientId=retailwebsite&quot;);
            params.push(&quot;deviceType=web&quot;);
            params.push(&quot;showFeatures=deliveryBlock&quot;);
            params.push(&quot;ie=UTF8&quot;);
            params.push(&quot;experienceId=deliveryBlockQuantityRefreshAjaxExperience&quot;);

            // Weblab gated addition of Locale and OfferListingId to QuantityRefresh request
            var addLParamsToQuantityRefreshWeblabFlag = false;
            if (addLParamsToQuantityRefreshWeblabFlag) {
                var locale = accordionRow.find(&quot;#deliveryBlockSelectLocale&quot;).val();
                var offerListingId = accordionRow.find(&quot;#deliveryBlockSelectOfferListingId&quot;).val();

                // Only add language param if locale is non-null
                if (locale) {
                    params.push(&quot;language=&quot; + locale);
                }
                // Only add offerListingId param if value is non-null
                if (offerListingId) {
                    params.push(&quot;offerListingId=&quot; + offerListingId);
                }
            }

            // Weblab gated omboarding of MOD to UDM
            var onboardModToUdmWwWeblabTreatmentWeblabFlag = false;

            if (onboardModToUdmWwWeblabTreatmentWeblabFlag) {
                $.ajax({
                   type: &quot;GET&quot;,
                   url: &quot;/gp/product/ajax?&quot;,
                   contentType: &quot; , &quot;'&quot; , &quot;application/x-www-form-urlencoded;charset=utf-8&quot; , &quot;'&quot; , &quot;,
                   data: params.join(&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;),
                   accordionRow: accordionRow,
                   dataType: &quot;html&quot;,
                   success: function(objResponse) {
                    // add weblab gating?
                       if (objResponse != null &amp;&amp; objResponse != &quot;&quot;) {
                            // parse objResponse and extract it into DB Small and DB Large

                             // The specific string to split the HTML content
                             // Split the HTML content using the separator
                             var htmlContentArray = objResponse.split(&quot;##split##&quot;);

                             var objResponseDbLarge = htmlContentArray[0];
                             var objResponseDbSmall = htmlContentArray[1];

                             // null checks
                             if (objResponseDbLarge != null &amp;&amp; objResponseDbLarge != &quot;&quot;){
                                 accordionRow.find(&quot;#deliveryBlockMessage&quot;).replaceWith(objResponseDbLarge);
                             }
                             if (objResponseDbSmall != null &amp;&amp; objResponseDbSmall != &quot;&quot;){
                                 accordionRow.find(&quot;#deliveryBlockSmallMessage&quot;).replaceWith(objResponseDbSmall);
                             }
                       }
                   }
                });
            } else {
                $.ajax({
                    type: &quot;GET&quot;,
                    url: &quot;/gp/product/ajax?&quot;,
                    contentType: &quot; , &quot;'&quot; , &quot;application/x-www-form-urlencoded;charset=utf-8&quot; , &quot;'&quot; , &quot;,
                    data: params.join(&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;),
                    accordionRow: accordionRow,
                    dataType: &quot;html&quot;,
                    success: function(objResponse) {
                        if (objResponse != null &amp;&amp; objResponse != &quot;&quot;) {
                            accordionRow.find(&quot;#deliveryBlockMessage&quot;).replaceWith(objResponse);
                        }
                    }
               });
            }
            return;
        });
    });
                             
                                  
                                    
                                
                                
                                    Deliver to India
                                
                            
                                                          
                               
                            
                                                                        
                            
                                                                                       
                            
                                                                                                                      
                            
                           (function(f) {var _np=(window.P._namespace(&quot;promoRenameBuyBoxCXCW&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
    P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;jQuery&quot; , &quot;'&quot; , &quot;).execute(function(A, $) {
        $(&quot; , &quot;'&quot; , &quot;#desktop_buybox&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;#promoPriceBlockMessage_feature_div&quot; , &quot;'&quot; , &quot;).prop(&quot;id&quot;,&quot;promoPriceBlockMessageInBuyBox_feature_div&quot;);
    });
}));                           
                             
                                  
                                                 
                                  
                                                   
                                  
                                                                                                              Only 20 left in stock - order soon.                                                            
    .availabilityMoreDetailsIcon {
        width: 12px;
        vertical-align: baseline;
        fill: #969696;
    }
                              
                                  
                                                                               
                                  
                                                     
                                  
                                                                       
                                  
                                                    
    /* Adding this CSS to overridden the green badging styles */
    #bmsmMessaging span.a-text-bold {
        font-weight: normal !important;
        background-color: #7fda69;
        padding: 2px 6px;
    }

    #bmsmMessaging {
        font-weight: normal !important;
        margin-bottom: 12px !important;
        text-align: left;
        display:  none;
    }

    /* Only display quantity discount on qualified buy box; excluding pickup and other buy box */
    #qualifiedBuybox #bmsmMessaging {
        display: block !important;
    }

    #bmsmMessaging .a-icon.a-icon-popover {
        margin-top: 6px !important;
    }

    #bmsmMessaging .a-color-success {
        color: black !important;
    }

    @media (min-width: 801px){
        #bmsmMessaging span{
            font-size: 14px !important;
        }
    }

    /* mobile screen */
    @media (max-width: 800px){
        #bmsmMessaging span{
            font-size: 15px !important;
        }

        #bmsmMessaging span.a-text-bold {
            margin-right: 5px;
        }
    }


                                                                                                                      
			     Quantity:     1        2        3        4        5        6        7+     Quantity:1                      
        P.when(&quot; , &quot;'&quot; , &quot;quantity-restriction-messaging&quot; , &quot;'&quot; , &quot;).execute(function(quantityRestrictionMessaging){
            quantityRestrictionMessaging.quantityModal({&quot;popoverTitle&quot;:&quot;Your Device Order&quot;,&quot;moq&quot;:7,&quot;message&quot;:&quot;&lt;div data-csa-c-type=widget data-csa-c-slot-id=qrm-div data-csa-c-content-id=qrm-devices>Standard Amazon accounts are limited to fewer than MAX_QUANTITY_PLUS_ONE units.  Larger quantities may be ordered with an &lt;a href=\&quot;/b/?ie=UTF8&amp;node=23544440011\&quot;>Amazon Business&lt;\/a> account. Sign in with or create an Amazon Business account to continue. Amazon does not offer retail promotional discounts for bulk device orders and all bundle offers involving Amazon devices are limited to one per customer.&lt;\/div>&quot;,&quot;cspMessage&quot;:&quot;To discuss options for quantities higher than MAX_QUANTITY_PLUS_ONE units, please &lt;a href=\&quot;mailto:amazon-bulk-device-sales@amazon.com?subject=Amazon%20Bulk%20Device%20Request%20-%20Negotiated%20price&amp;body=Amazon%20is%20looking%20forward%20to%20supporting%20your%20negotiated%20price%20device%20request.%0d%0dPlease%20provide%20the%20following%20information%20so%20your%20account%20manager%20can%20better%20assist%20you. %0d%0dFirst%20and%20Last%20Name:%20%0dPhone%20Number:%20%0dBusiness%20Name:%20%0dWhich%20device(s)%20are%20you%20seeking?%20%0dWhat%20quantity%20are%20you%20seeking?%20%0dWho%20and%20where%20are%20the%20end%20customers?%20%0dWhen%20do%20you%20need%20devices%20in%20hand?%20%0d\&quot;>email your account manager.&lt;\/a> &quot;,&quot;cspMOQ&quot;:&quot;1000&quot;})
        });
     
                                             
                                  
                                                            
                                  
                                      
    
    

           $$48.9948.99  
                    ()
              Includes selected options.   Includes initial monthly payment and selected options.          
                         Details  
                          Price     ($48.99x)          $48.99          Subtotal     $$48.9948.99      Subtotal                  Initial payment breakdown           Shipping cost, delivery date, and order total (including tax) shown at checkout. 
                                                    
                                  
                                                     
                                  
                                                         
                                  
                           
             Similar items shipping to India
              IN
                India
             
        
         if(window.mix_csa){window.mix_csa(&quot; , &quot;'&quot; , &quot;[cel_widget_id=&quot;export-alternative-card-desktop_desktop-export-alternative_0&quot;]&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#CardInstancemvDQsFXTLA2hZ3IKqUIS9Q&quot; , &quot;'&quot; , &quot;)(&quot; , &quot;'&quot; , &quot;mark&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;bb&quot; , &quot;'&quot; , &quot;)}
if(window.uet){window.uet(&quot; , &quot;'&quot; , &quot;bb&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;export-alternative-card-desktop_desktop-export-alternative_0&quot; , &quot;'&quot; , &quot;,{wb: 1})}
._export-alternative-card-desktop_style_export_alternative_table_desktop__2ehnZ{background:none!important;background-color:none!important;width:99%}._export-alternative-card-desktop_style_export_alternative_table_first_empty_col__3xZRw,._export-alternative-card-desktop_style_export_alternative_table_mobile__3hpwj{background:none!important;background-color:none!important}._export-alternative-card-desktop_style_export_alternative_table_first_empty_col__3xZRw{width:160px}._export-alternative-card-desktop_style_export_alternative_table_first_label_col__ivoXZ{background-color:#e0e0e0!important;padding-top:5px!important;width:160px}._export-alternative-card-desktop_style_export_alternative_table_image_title_row__31u-4{background:none!important;background-color:none!important;border-left-style:hidden;border-right-style:hidden;border-top-style:hidden}._export-alternative-card-desktop_style_export_alternative_table_image_title_cell__2L2zL{background:none!important;background-color:none!important;border-style:hidden}._export-alternative-card-desktop_style_export_alternative_table_cta_row__3lFCu{background:none!important;background-color:none!important;border-bottom-style:hidden;border-left-style:hidden;border-right-style:hidden}._export-alternative-card-desktop_style_export_alternative_table_cta_cell__1c3f_{background:none!important;background-color:none!important;border-style:hidden}._export-alternative-card-desktop_style_export_alternative_energy_efficiency_cell__2667h{padding-right:5px!important;padding-top:10px!important}._export-alternative-card-desktop_style_export_alternative_in_cell_postion__1M5sG{padding-right:15px!important;padding-top:5px!important}._export-alternative-card-desktop_style_export_alternative_shipping_fee_row__C-mis{height:120px!important}
._export-alternative-card-desktop_style_export_alternative_cta_button__22AJK{width:100%}._export-alternative-card-desktop_style_export_alternative_image_shift__rrAwZ{padding-left:25%!important}._export-alternative-card-desktop_style_export_alternative_energy_efficiency_desktop__2zh3v{background:none!important;background-color:none!important;margin:0;overflow:hidden;padding:0}._export-alternative-card-desktop_style_export_alternative_energy_efficiency_mobile__3Z6NM{background:none!important;background-color:none!important;margin:0;overflow:hidden;padding:0;position:relative}
._export-alternative-card-desktop_energy-efficiency_energy-efficiency-container__1Pkva{text-align:left}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-standard__28gp8{cursor:pointer;display:inline-block;height:24px}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-shape__1IcJY{display:inline-block;height:24px}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-rating__3_0eN{fill:#fff;font-size:20px;vertical-align:middle}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK{fill:#fff;font-size:14px;vertical-align:middle}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P{left:24px * .6;text-shadow:-.5px -.5px 0 #000,.5px -.5px 0 #000,-.5px .5px 0 #000,.5px .5px 0 #000}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2{display:inline-block;padding-left:5px;padding-top:0;position:absolute;vertical-align:middle}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3{cursor:pointer;word-break:break-word}
._export-alternative-card-desktop_style_export_alternative_trigger_button__3e7gX{width:100%}
See Similar Itemsif(window.mix_csa){window.mix_csa(&quot; , &quot;'&quot; , &quot;[cel_widget_id=&quot;export-alternative-card-desktop_desktop-export-alternative_0&quot;]&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#CardInstancemvDQsFXTLA2hZ3IKqUIS9Q&quot; , &quot;'&quot; , &quot;)(&quot; , &quot;'&quot; , &quot;mark&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;be&quot; , &quot;'&quot; , &quot;)}
if(window.uet){window.uet(&quot; , &quot;'&quot; , &quot;be&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;export-alternative-card-desktop_desktop-export-alternative_0&quot; , &quot;'&quot; , &quot;,{wb: 1})}
if(window.mixTimeout){window.mixTimeout(&quot; , &quot;'&quot; , &quot;export-alternative-card-desktop&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CardInstancemvDQsFXTLA2hZ3IKqUIS9Q&quot; , &quot;'&quot; , &quot;, 90000)};
P.when(&quot; , &quot;'&quot; , &quot;mix:@amzn/mix.client-runtime&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mix:export-alternative-card-desktop__OD63ZHOz&quot; , &quot;'&quot; , &quot;).execute(function (runtime, cardModule) {runtime.registerCardFactory(&quot; , &quot;'&quot; , &quot;CardInstancemvDQsFXTLA2hZ3IKqUIS9Q&quot; , &quot;'&quot; , &quot;, cardModule).then(function(){if(window.mix_csa){window.mix_csa(&quot; , &quot;'&quot; , &quot;[cel_widget_id=&quot;export-alternative-card-desktop_desktop-export-alternative_0&quot;]&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#CardInstancemvDQsFXTLA2hZ3IKqUIS9Q&quot; , &quot;'&quot; , &quot;)(&quot; , &quot;'&quot; , &quot;mark&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;functional&quot; , &quot;'&quot; , &quot;)}if(window.uex){window.uex(&quot; , &quot;'&quot; , &quot;ld&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;export-alternative-card-desktop_desktop-export-alternative_0&quot; , &quot;'&quot; , &quot;,{wb: 1})}});});

P.load.js(&quot; , &quot;'&quot; , &quot;https://images-na.ssl-images-amazon.com/images/I/31PCZfmrW-L.js?xcp&quot; , &quot;'&quot; , &quot;);

                             
                                  
                                                                                                                                          {&quot;shouldUseNatc&quot;:true}                                    
                             Add to Cart            
  


This item cannot be shipped to your selected delivery location. Please choose a different delivery location.

       
                                                 
                                  
                                                                            
                                  
                                 Enhancements you chose aren&quot; , &quot;'&quot; , &quot;t available for this seller.          Details         To add the following enhancements to your purchase, choose a different seller.     %cardName%         ${cardName} not available for the seller you chose     ${cardName} unavailable for quantities greater than ${maxQuantity}.                                
                                  
                                                    
                                  
                                                               
                                                         
                                                        
                                  
                                 
    
        Ships from 


           
    Amazon.com         
       Ships from             Amazon.com                                      
                                  
                                 
    
        Sold by 


           
    Amazon.com         
       Sold by             Amazon.com                                      
                                  
                                 
    
        Returns 


                  Eligible for Return, Refund or Replacement within 30 days of receipt        Eligible for Return, Refund or Replacement within 30 days of receipt    This item can be returned in its original condition for a full refund or replacement within 30 days of receipt.      Read full return policy              
       Returns                Eligible for Return, Refund or Replacement within 30 days of receipt    This item can be returned in its original condition for a full refund or replacement within 30 days of receipt.      Read full return policy                                          
                                  
                                                        
                                  
                                                     
                                  
                                                     
                                  
                                                     
                                  
                                                      
                                  
                                                     
                                  
                                                     
                                  
                                                        
                                  
                              
                                    
                                                       
                                     
                                                       
                                     
                                                       
                                     
                                                       
                                     
                                       
    
        Payment 


                  Secure transaction        Your transaction is secure    We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.   Learn more               
       Payment                Secure transaction    We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.   Learn more                                            
                                     
                                                        
                                     
                                                       
         
                      
          
                                    Details      See more            
               
                                  
                                  
                                                                     
                                  
                                                                       
                                  
                                                                          
                                  
                                                      
                                  
                                                    
                                  
                                                    
                                  
                                                                     
                                  
                                                      
                                  
                                                     
                                  
                                                       
                                  
                                                       
                                  
                                                    
                                  
                                                          
                                  
                                                     
                                  
                                                     
                                  
                                                       
                                  
                                                     
                                  
                                                     
                                  
                                                    
                                  
                          {&quot;heroName&quot;:&quot;&quot;} {}                             
                                  
                                                    
                                  
                                                     
                                  
                                                    
                                  
                                                                                      
                                  
                               This is a gift                                   
                                  
                         
 
        Link device to your Amazon account   to simplify setup.        Why is this important?      Link this device to your Amazon account to enable Frustration-Free Setup.   If you already have a qualifying Amazon device connected to your home network and stored network credentials, the new device can automatically join the same home network and automatically complete device setup.   Learn more about Amazon Frustration-Free Setup         You’ve marked this as a gift, so it will not be linked to your account. To link to your account, first de-select &quot;This is a gift&quot;   Account linking is not available when ordering more than 3 of this device.     (function(f) {var _np=(window.P._namespace(&quot;PreRegistration&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
        P.when(&quot;PreRegistration.Controller&quot;, &quot;ready&quot;).execute(function(PreRegistrationController) {
        var controllerDataJson = {&quot;maxQuantity&quot;:3,&quot;defaultValue&quot;:true,&quot;showStatus&quot;:true,&quot;asin&quot;:&quot;B085M5XQVR&quot;,&quot;status&quot;:&quot;ALLOWED_CONDITIONAL&quot;};
        PreRegistrationController.getInstance(controllerDataJson);
        });
    }));                        
                                  
                                                 
                                  
                                                      
                                  
                                                     
          &quot;) or . = concat(&quot;                               
                                                       
                                  
                                                    
                                  
                                                                 
                                                      
                            
                                                    
                            
                                                
                            
                                                                               
                            
                                 
                             
                            
                                                      
                            
                                                                                                                               
                            
                                                                 
                            
                                                                 
                            
                                                   
                                    
  


This item cannot be shipped to your selected delivery location. Please choose a different delivery location.

               
  
    
    P.when(&quot;A&quot;, &quot;jQuery&quot;).execute(function(A, $) {
        $(&quot;#selectQuantity [name=&quot; , &quot;'&quot; , &quot;quantity&quot; , &quot;'&quot; , &quot;], #mobileQuantityDropDown&quot;).live(&quot;change&quot;, function (event) {

            if (event.updateDeliveryBlockOnQuantityChange) {
                return;
            }
            event.updateDeliveryBlockOnQuantityChange = 1;

            // &quot;#buybox&quot; is included in this list because if there is no accordion row, then it is a single-offer layout
            // possible id&quot; , &quot;'&quot; , &quot;s may include &quot;usedAccordionRow&quot;, &quot;newAccordionRow_1&quot;, &quot;newAccordionRow_2&quot;
            var accordionRow = $(this).closest(&quot; , &quot;'&quot; , &quot;[id$=&quot;AccordionRow&quot;], #buybox, [id^=&quot;newAccordionRow&quot;]&quot; , &quot;'&quot; , &quot;);

            var quantity = $(this).val();
            var asin = accordionRow.find(&quot;#deliveryBlockSelectAsin&quot;).val();
            var merchantId = accordionRow.find(&quot;#deliveryBlockSelectMerchant&quot;).val();

            if (!asin || !merchantId) {
                return;
            }

            var params = [];

            params.push(&quot;asin=&quot; + asin);
            params.push(&quot;quantity=&quot; + quantity);
            params.push(&quot;exclusiveMerchantId=&quot; + merchantId);
            params.push(&quot;merchantId=&quot; + merchantId);
            params.push(&quot;clientId=retailwebsite&quot;);
            params.push(&quot;deviceType=web&quot;);
            params.push(&quot;showFeatures=deliveryBlock&quot;);
            params.push(&quot;ie=UTF8&quot;);
            params.push(&quot;experienceId=deliveryBlockQuantityRefreshAjaxExperience&quot;);

            // Weblab gated addition of Locale and OfferListingId to QuantityRefresh request
            var addLParamsToQuantityRefreshWeblabFlag = false;
            if (addLParamsToQuantityRefreshWeblabFlag) {
                var locale = accordionRow.find(&quot;#deliveryBlockSelectLocale&quot;).val();
                var offerListingId = accordionRow.find(&quot;#deliveryBlockSelectOfferListingId&quot;).val();

                // Only add language param if locale is non-null
                if (locale) {
                    params.push(&quot;language=&quot; + locale);
                }
                // Only add offerListingId param if value is non-null
                if (offerListingId) {
                    params.push(&quot;offerListingId=&quot; + offerListingId);
                }
            }

            // Weblab gated omboarding of MOD to UDM
            var onboardModToUdmWwWeblabTreatmentWeblabFlag = false;

            if (onboardModToUdmWwWeblabTreatmentWeblabFlag) {
                $.ajax({
                   type: &quot;GET&quot;,
                   url: &quot;/gp/product/ajax?&quot;,
                   contentType: &quot; , &quot;'&quot; , &quot;application/x-www-form-urlencoded;charset=utf-8&quot; , &quot;'&quot; , &quot;,
                   data: params.join(&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;),
                   accordionRow: accordionRow,
                   dataType: &quot;html&quot;,
                   success: function(objResponse) {
                    // add weblab gating?
                       if (objResponse != null &amp;&amp; objResponse != &quot;&quot;) {
                            // parse objResponse and extract it into DB Small and DB Large

                             // The specific string to split the HTML content
                             // Split the HTML content using the separator
                             var htmlContentArray = objResponse.split(&quot;##split##&quot;);

                             var objResponseDbLarge = htmlContentArray[0];
                             var objResponseDbSmall = htmlContentArray[1];

                             // null checks
                             if (objResponseDbLarge != null &amp;&amp; objResponseDbLarge != &quot;&quot;){
                                 accordionRow.find(&quot;#deliveryBlockMessage&quot;).replaceWith(objResponseDbLarge);
                             }
                             if (objResponseDbSmall != null &amp;&amp; objResponseDbSmall != &quot;&quot;){
                                 accordionRow.find(&quot;#deliveryBlockSmallMessage&quot;).replaceWith(objResponseDbSmall);
                             }
                       }
                   }
                });
            } else {
                $.ajax({
                    type: &quot;GET&quot;,
                    url: &quot;/gp/product/ajax?&quot;,
                    contentType: &quot; , &quot;'&quot; , &quot;application/x-www-form-urlencoded;charset=utf-8&quot; , &quot;'&quot; , &quot;,
                    data: params.join(&quot; , &quot;'&quot; , &quot;&amp;&quot; , &quot;'&quot; , &quot;),
                    accordionRow: accordionRow,
                    dataType: &quot;html&quot;,
                    success: function(objResponse) {
                        if (objResponse != null &amp;&amp; objResponse != &quot;&quot;) {
                            accordionRow.find(&quot;#deliveryBlockMessage&quot;).replaceWith(objResponse);
                        }
                    }
               });
            }
            return;
        });
    });
                             
                                  
                                    
                                
                                
                                    Deliver to India
                                
                            
                                                          
                               
                            
                                                                        
                            
                                                                                       
                            
                                                                                                                      
                            
                           (function(f) {var _np=(window.P._namespace(&quot;promoRenameBuyBoxCXCW&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
    P.when(&quot; , &quot;'&quot; , &quot;A&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;jQuery&quot; , &quot;'&quot; , &quot;).execute(function(A, $) {
        $(&quot; , &quot;'&quot; , &quot;#desktop_buybox&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;#promoPriceBlockMessage_feature_div&quot; , &quot;'&quot; , &quot;).prop(&quot;id&quot;,&quot;promoPriceBlockMessageInBuyBox_feature_div&quot;);
    });
}));                           
                             
                                  
                                                 
                                  
                                                   
                                  
                                                                                                              Only 20 left in stock - order soon.                                                            
    .availabilityMoreDetailsIcon {
        width: 12px;
        vertical-align: baseline;
        fill: #969696;
    }
                              
                                  
                                                                               
                                  
                                                     
                                  
                                                                       
                                  
                                                    
    /* Adding this CSS to overridden the green badging styles */
    #bmsmMessaging span.a-text-bold {
        font-weight: normal !important;
        background-color: #7fda69;
        padding: 2px 6px;
    }

    #bmsmMessaging {
        font-weight: normal !important;
        margin-bottom: 12px !important;
        text-align: left;
        display:  none;
    }

    /* Only display quantity discount on qualified buy box; excluding pickup and other buy box */
    #qualifiedBuybox #bmsmMessaging {
        display: block !important;
    }

    #bmsmMessaging .a-icon.a-icon-popover {
        margin-top: 6px !important;
    }

    #bmsmMessaging .a-color-success {
        color: black !important;
    }

    @media (min-width: 801px){
        #bmsmMessaging span{
            font-size: 14px !important;
        }
    }

    /* mobile screen */
    @media (max-width: 800px){
        #bmsmMessaging span{
            font-size: 15px !important;
        }

        #bmsmMessaging span.a-text-bold {
            margin-right: 5px;
        }
    }


                                                                                                                      
			     Quantity:     1        2        3        4        5        6        7+     Quantity:1                      
        P.when(&quot; , &quot;'&quot; , &quot;quantity-restriction-messaging&quot; , &quot;'&quot; , &quot;).execute(function(quantityRestrictionMessaging){
            quantityRestrictionMessaging.quantityModal({&quot;popoverTitle&quot;:&quot;Your Device Order&quot;,&quot;moq&quot;:7,&quot;message&quot;:&quot;&lt;div data-csa-c-type=widget data-csa-c-slot-id=qrm-div data-csa-c-content-id=qrm-devices>Standard Amazon accounts are limited to fewer than MAX_QUANTITY_PLUS_ONE units.  Larger quantities may be ordered with an &lt;a href=\&quot;/b/?ie=UTF8&amp;node=23544440011\&quot;>Amazon Business&lt;\/a> account. Sign in with or create an Amazon Business account to continue. Amazon does not offer retail promotional discounts for bulk device orders and all bundle offers involving Amazon devices are limited to one per customer.&lt;\/div>&quot;,&quot;cspMessage&quot;:&quot;To discuss options for quantities higher than MAX_QUANTITY_PLUS_ONE units, please &lt;a href=\&quot;mailto:amazon-bulk-device-sales@amazon.com?subject=Amazon%20Bulk%20Device%20Request%20-%20Negotiated%20price&amp;body=Amazon%20is%20looking%20forward%20to%20supporting%20your%20negotiated%20price%20device%20request.%0d%0dPlease%20provide%20the%20following%20information%20so%20your%20account%20manager%20can%20better%20assist%20you. %0d%0dFirst%20and%20Last%20Name:%20%0dPhone%20Number:%20%0dBusiness%20Name:%20%0dWhich%20device(s)%20are%20you%20seeking?%20%0dWhat%20quantity%20are%20you%20seeking?%20%0dWho%20and%20where%20are%20the%20end%20customers?%20%0dWhen%20do%20you%20need%20devices%20in%20hand?%20%0d\&quot;>email your account manager.&lt;\/a> &quot;,&quot;cspMOQ&quot;:&quot;1000&quot;})
        });
     
                                             
                                  
                                                            
                                  
                                      
    
    

           $$48.9948.99  
                    ()
              Includes selected options.   Includes initial monthly payment and selected options.          
                         Details  
                          Price     ($48.99x)          $48.99          Subtotal     $$48.9948.99      Subtotal                  Initial payment breakdown           Shipping cost, delivery date, and order total (including tax) shown at checkout. 
                                                    
                                  
                                                     
                                  
                                                         
                                  
                           
             Similar items shipping to India
              IN
                India
             
        
         if(window.mix_csa){window.mix_csa(&quot; , &quot;'&quot; , &quot;[cel_widget_id=&quot;export-alternative-card-desktop_desktop-export-alternative_0&quot;]&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#CardInstancemvDQsFXTLA2hZ3IKqUIS9Q&quot; , &quot;'&quot; , &quot;)(&quot; , &quot;'&quot; , &quot;mark&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;bb&quot; , &quot;'&quot; , &quot;)}
if(window.uet){window.uet(&quot; , &quot;'&quot; , &quot;bb&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;export-alternative-card-desktop_desktop-export-alternative_0&quot; , &quot;'&quot; , &quot;,{wb: 1})}
._export-alternative-card-desktop_style_export_alternative_table_desktop__2ehnZ{background:none!important;background-color:none!important;width:99%}._export-alternative-card-desktop_style_export_alternative_table_first_empty_col__3xZRw,._export-alternative-card-desktop_style_export_alternative_table_mobile__3hpwj{background:none!important;background-color:none!important}._export-alternative-card-desktop_style_export_alternative_table_first_empty_col__3xZRw{width:160px}._export-alternative-card-desktop_style_export_alternative_table_first_label_col__ivoXZ{background-color:#e0e0e0!important;padding-top:5px!important;width:160px}._export-alternative-card-desktop_style_export_alternative_table_image_title_row__31u-4{background:none!important;background-color:none!important;border-left-style:hidden;border-right-style:hidden;border-top-style:hidden}._export-alternative-card-desktop_style_export_alternative_table_image_title_cell__2L2zL{background:none!important;background-color:none!important;border-style:hidden}._export-alternative-card-desktop_style_export_alternative_table_cta_row__3lFCu{background:none!important;background-color:none!important;border-bottom-style:hidden;border-left-style:hidden;border-right-style:hidden}._export-alternative-card-desktop_style_export_alternative_table_cta_cell__1c3f_{background:none!important;background-color:none!important;border-style:hidden}._export-alternative-card-desktop_style_export_alternative_energy_efficiency_cell__2667h{padding-right:5px!important;padding-top:10px!important}._export-alternative-card-desktop_style_export_alternative_in_cell_postion__1M5sG{padding-right:15px!important;padding-top:5px!important}._export-alternative-card-desktop_style_export_alternative_shipping_fee_row__C-mis{height:120px!important}
._export-alternative-card-desktop_style_export_alternative_cta_button__22AJK{width:100%}._export-alternative-card-desktop_style_export_alternative_image_shift__rrAwZ{padding-left:25%!important}._export-alternative-card-desktop_style_export_alternative_energy_efficiency_desktop__2zh3v{background:none!important;background-color:none!important;margin:0;overflow:hidden;padding:0}._export-alternative-card-desktop_style_export_alternative_energy_efficiency_mobile__3Z6NM{background:none!important;background-color:none!important;margin:0;overflow:hidden;padding:0;position:relative}
._export-alternative-card-desktop_energy-efficiency_energy-efficiency-container__1Pkva{text-align:left}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-standard__28gp8{cursor:pointer;display:inline-block;height:24px}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-shape__1IcJY{display:inline-block;height:24px}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-rating__3_0eN{fill:#fff;font-size:20px;vertical-align:middle}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-rating-sign__1ronK{fill:#fff;font-size:14px;vertical-align:middle}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-rating-2021__2Q_3P{left:24px * .6;text-shadow:-.5px -.5px 0 #000,.5px -.5px 0 #000,-.5px .5px 0 #000,.5px .5px 0 #000}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-data-sheet-label-container__2iEi2{display:inline-block;padding-left:5px;padding-top:0;position:absolute;vertical-align:middle}._export-alternative-card-desktop_energy-efficiency_energy-efficiency-badge-data-sheet-label__3b6X3{cursor:pointer;word-break:break-word}
._export-alternative-card-desktop_style_export_alternative_trigger_button__3e7gX{width:100%}
See Similar Itemsif(window.mix_csa){window.mix_csa(&quot; , &quot;'&quot; , &quot;[cel_widget_id=&quot;export-alternative-card-desktop_desktop-export-alternative_0&quot;]&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#CardInstancemvDQsFXTLA2hZ3IKqUIS9Q&quot; , &quot;'&quot; , &quot;)(&quot; , &quot;'&quot; , &quot;mark&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;be&quot; , &quot;'&quot; , &quot;)}
if(window.uet){window.uet(&quot; , &quot;'&quot; , &quot;be&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;export-alternative-card-desktop_desktop-export-alternative_0&quot; , &quot;'&quot; , &quot;,{wb: 1})}
if(window.mixTimeout){window.mixTimeout(&quot; , &quot;'&quot; , &quot;export-alternative-card-desktop&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CardInstancemvDQsFXTLA2hZ3IKqUIS9Q&quot; , &quot;'&quot; , &quot;, 90000)};
P.when(&quot; , &quot;'&quot; , &quot;mix:@amzn/mix.client-runtime&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mix:export-alternative-card-desktop__OD63ZHOz&quot; , &quot;'&quot; , &quot;).execute(function (runtime, cardModule) {runtime.registerCardFactory(&quot; , &quot;'&quot; , &quot;CardInstancemvDQsFXTLA2hZ3IKqUIS9Q&quot; , &quot;'&quot; , &quot;, cardModule).then(function(){if(window.mix_csa){window.mix_csa(&quot; , &quot;'&quot; , &quot;[cel_widget_id=&quot;export-alternative-card-desktop_desktop-export-alternative_0&quot;]&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#CardInstancemvDQsFXTLA2hZ3IKqUIS9Q&quot; , &quot;'&quot; , &quot;)(&quot; , &quot;'&quot; , &quot;mark&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;functional&quot; , &quot;'&quot; , &quot;)}if(window.uex){window.uex(&quot; , &quot;'&quot; , &quot;ld&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;export-alternative-card-desktop_desktop-export-alternative_0&quot; , &quot;'&quot; , &quot;,{wb: 1})}});});

P.load.js(&quot; , &quot;'&quot; , &quot;https://images-na.ssl-images-amazon.com/images/I/31PCZfmrW-L.js?xcp&quot; , &quot;'&quot; , &quot;);

                             
                                  
                                                                                                                                          {&quot;shouldUseNatc&quot;:true}                                    
                             Add to Cart            
  


This item cannot be shipped to your selected delivery location. Please choose a different delivery location.

       
                                                 
                                  
                                                                            
                                  
                                 Enhancements you chose aren&quot; , &quot;'&quot; , &quot;t available for this seller.          Details         To add the following enhancements to your purchase, choose a different seller.     %cardName%         ${cardName} not available for the seller you chose     ${cardName} unavailable for quantities greater than ${maxQuantity}.                                
                                  
                                                    
                                  
                                                               
                                                         
                                                        
                                  
                                 
    
        Ships from 


           
    Amazon.com         
       Ships from             Amazon.com                                      
                                  
                                 
    
        Sold by 


           
    Amazon.com         
       Sold by             Amazon.com                                      
                                  
                                 
    
        Returns 


                  Eligible for Return, Refund or Replacement within 30 days of receipt        Eligible for Return, Refund or Replacement within 30 days of receipt    This item can be returned in its original condition for a full refund or replacement within 30 days of receipt.      Read full return policy              
       Returns                Eligible for Return, Refund or Replacement within 30 days of receipt    This item can be returned in its original condition for a full refund or replacement within 30 days of receipt.      Read full return policy                                          
                                  
                                                        
                                  
                                                     
                                  
                                                     
                                  
                                                     
                                  
                                                      
                                  
                                                     
                                  
                                                     
                                  
                                                        
                                  
                              
                                    
                                                       
                                     
                                                       
                                     
                                                       
                                     
                                                       
                                     
                                       
    
        Payment 


                  Secure transaction        Your transaction is secure    We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.   Learn more               
       Payment                Secure transaction    We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.   Learn more                                            
                                     
                                                        
                                     
                                                       
         
                      
          
                                    Details      See more            
               
                                  
                                  
                                                                     
                                  
                                                                       
                                  
                                                                          
                                  
                                                      
                                  
                                                    
                                  
                                                    
                                  
                                                                     
                                  
                                                      
                                  
                                                     
                                  
                                                       
                                  
                                                       
                                  
                                                    
                                  
                                                          
                                  
                                                     
                                  
                                                     
                                  
                                                       
                                  
                                                     
                                  
                                                     
                                  
                                                    
                                  
                          {&quot;heroName&quot;:&quot;&quot;} {}                             
                                  
                                                    
                                  
                                                     
                                  
                                                    
                                  
                                                                                      
                                  
                               This is a gift                                   
                                  
                         
 
        Link device to your Amazon account   to simplify setup.        Why is this important?      Link this device to your Amazon account to enable Frustration-Free Setup.   If you already have a qualifying Amazon device connected to your home network and stored network credentials, the new device can automatically join the same home network and automatically complete device setup.   Learn more about Amazon Frustration-Free Setup         You’ve marked this as a gift, so it will not be linked to your account. To link to your account, first de-select &quot;This is a gift&quot;   Account linking is not available when ordering more than 3 of this device.     (function(f) {var _np=(window.P._namespace(&quot;PreRegistration&quot;));if(_np.guardFatal){_np.guardFatal(f)(_np);}else{f(_np);}}(function(P) {
        P.when(&quot;PreRegistration.Controller&quot;, &quot;ready&quot;).execute(function(PreRegistrationController) {
        var controllerDataJson = {&quot;maxQuantity&quot;:3,&quot;defaultValue&quot;:true,&quot;showStatus&quot;:true,&quot;asin&quot;:&quot;B085M5XQVR&quot;,&quot;status&quot;:&quot;ALLOWED_CONDITIONAL&quot;};
        PreRegistrationController.getInstance(controllerDataJson);
        });
    }));                        
                                  
                                                 
                                  
                                                      
                                  
                                                     
          &quot;))]</value>
      <webElementGuid>aaaf1e44-6a76-49e5-892a-976d4fe082b6</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
