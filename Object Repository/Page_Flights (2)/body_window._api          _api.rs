<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_window._api          _api</name>
   <tag></tag>
   <elementGuidId>78ad7b6f-16f5-4adc-8f94-95678dbf74bd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>ts-in-i-frame ltr</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>dir</name>
      <type>Main</type>
      <value>ltr</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>ng-class</name>
      <type>Main</type>
      <value>[{ 'has-modal': hasModal, 'has-overlay-modal': hasOverlayModal, 'has-drawer': hasDrawer, 'ts-in-i-frame' : isInIFrame}, ft_responsive_design &amp;&amp; pageName ? pageName + '-page' : '']</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

    
    
        window._api = {};

        _api.call = function (obj) {
            if ( obj &amp;&amp; _api.isJson(obj) ) {
                var domElement = document.getElementById('api');
                var scope = angular.element(domElement).scope();
                scope.$apply(function(){
                    scope.pass = obj;
                });
                return true;
            }
            console.log('api call fail');
            return false;
        }

        _api.isJson = function (str) {
            try {
                var o = JSON.parse(str);

                // Handle non-exception-throwing cases:
                // Neither JSON.parse(false) or JSON.parse(1234) throw errors, hence the type-checking,
                // but... JSON.parse(null) returns 'null', and typeof null === &quot;object&quot;,
                // so we must check for that, too.
                if (o &amp;&amp; typeof o === &quot;object&quot; &amp;&amp; o !== null) {
                    return o;
                }
            }
            catch (e) {}
            return false;
        }
    

    

    
        var dataLayer = window.dataLayer = window.dataLayer || {};
    

    
    
        function fareKeepCallback(data) {
            var obj = {};
            obj.action = &quot;fare-keep-callback&quot;;
            obj.data = data;
            _api.call(JSON.stringify(obj));
        }
    

    
    
        window.fbAsyncInit = function() {
            FB.init({
                appId            : '647547408712325',
                autoLogAppEvents : true,
                xfbml            : true,
                version          : 'v2.11',
                status           : true
            });
        };

        (function(d, s, id){
            var js, fjs = d.getElementsByTagName(s)[0];
            if (d.getElementById(id)) {return;}
            js = d.createElement(s); js.id = id;
            js.src = &quot;https://connect.facebook.net/en_US/sdk.js&quot;;
            fjs.parentNode.insertBefore(js, fjs);
        }(document, 'script', 'facebook-jssdk'));
    

    
    
    
        
        
    
    

        
            
                Search and Book Cheap Flights
            
            
            
            
    
        

        
            
                One way
                Return
                Multi city
            
        

        

            
            
                
                    
                        
                            
                            
                                From

                                
    
        

        
        
            
                
                    BWI
                    
                        Washington
                        United States
                    
                
            
        
    
        
    
    
    


                            
                            
                                To

                                
    
        

        
        
            
                
                    MKE
                    
                        Milwaukee, WI
                        United States
                    
                
            
        
    
        
    
    
    


                            
                        

                        
                            
                                
                                    
                                    
                                         
                                        
                                            Depart
                                            Date
                                        
                                    
                                
                            
                            
                            
                            
                                
                                
                                    
                                         
                                        
                                            One way
                                            Set return date
                                        
                                    
                                
                            
                            
                        
                    
                    
                        
                            
                            
                        
                        
                            
                        
                    
                
                
            

            
                
                    
                        1
                        
                            
                                Traveller
                            
                            
                                1 Adult
                                
                                
                            
                        
                    
                
                
            

            
                Search
            
        
        
            
                
                
                
                    More options
                
                
            

            
                

                
            
        
    
    




    
    
        
 
  
   
    image/svg+xml
    
    
   
  
 
 
  
   
  
 

        
        
            Choose your depart date
        
    
    
        
	

      
      

      
      

      May 2018

    

    
        
            SunMonTueWedThuFriSat
        
        
            
                
                	
                
                	
                
                	1
                
                	2
                
                	3
                
                	4
                
                	5
                
            
                
                	6
                
                	7
                
                	8
                
                	9
                
                	10
                
                	11
                
                	12
                
            
                
                	13
                
                	14
                
                	15
                
                	16
                
                	17
                
                	18
                
                	19
                
            
                
                	20
                
                	21
                
                	22
                
                	23
                
                	24
                
                	25
                
                	26
                
            
                
                	27
                
                	28
                
                	29
                
                	30
                
                	31
                
                	
                
                	
                
            
        
    

    

    
        

        

        
            
                Close
            
        
    



    

    


        
        
    

    


    
        
    

    

    

    

    

    

    

    










        
    

    

    


    
    

    

    
    
    
    

    
    

    
  

try {
(function(account, pagePath, profileId, rule){
function create(i, s, o, g, r, a, m){
  i[&quot;GoogleAnalyticsObject&quot;] = r; 

  i[r] = i[r] || function() {
    (i[r].q = i[r].q || []).push(arguments)
  },

    i[r].l = 1 * new Date();

  a = s.createElement(o),
    m = s.getElementsByTagName(o)[0];
  a.async = 1;
  a.src = g;
  m.parentNode.insertBefore(a, m)  
}
if (typeof(ga) == &quot;undefined&quot;) {
  create(window, document, &quot;script&quot;, &quot;//www.google-analytics.com/analytics.js&quot;, &quot;ga&quot;);
  ga(&quot;create&quot;, account, &quot;auto&quot;);
}
else if (ga.q) {
  ga.q.unshift([&quot;create&quot;, account, &quot;auto&quot;]);
  create(window, document, &quot;script&quot;, &quot;//www.google-analytics.com/analytics.js&quot;, &quot;ga&quot;);
}

// Set value for custom dimension Session ID
ga('set', 'dimension1', dataLayer.sessionId );

// Set value for custom dimension Innometrics Profile ID
ga('set', 'dimension2', profileId );

ga('set', 'page', pagePath);
ga(&quot;send&quot;, &quot;pageview&quot;);

})('UA-31275-56','/','xt2xaknspnbahq3psu9m74tfg8s6u7y4',document.getElementById(&quot;tag-container-jssuentl-1526271263795&quot;).rule);
} catch (e) {}try {
(function(accountId, hashedEmail, device, rule){
if (device &lt; 479) {
        device = &quot;m&quot;;
    } else if (device > 479 &amp;&amp; device &lt; 991) {
        device = &quot;t&quot;;
    } else {
        device = &quot;d&quot;;
    };

window.criteo_q = window.criteo_q || [];
        window.criteo_q.push(
            { event: &quot;setAccount&quot;, account: accountId },
          	{ event: &quot;setSiteType&quot;, type: device },
          	{ event: &quot;setHashedEmail&quot;, email: hashedEmail },
            { event: &quot;viewHome&quot; }
        );
})(13207,'',2560,document.getElementById(&quot;tag-container-adybfzcx-1526271263798&quot;).rule);
} catch (e) {}try {
(function(selector, prependContent, rule){
$(selector).prepend(prependContent);
})('.main','&lt;div id=&quot;personalisation-modal&quot; class=&quot;modal confirm-modal&quot;>&lt;div class=&quot;modal-dialog&quot;>&lt;div class=&quot;modal-content&quot;>&lt;div class=&quot;modal-header&quot;>&lt;button type=&quot;button&quot; class=&quot;close icon-close-circle blue&quot; data-dismiss=&quot;modal&quot; aria-hidden=&quot;true&quot;>&lt;/button>&lt;h3 id=&quot;myModalLabel&quot;>&lt;/h3>&lt;/div>&lt;div class=&quot;modal-body&quot;>&lt;p>&lt;/p>&lt;/div>&lt;/div>&lt;/div>&lt;/div>',document.getElementById(&quot;tag-container-zxvtfrkw-1526271263805&quot;).rule);
} catch (e) {}

var axel = Math.random() + &quot;&quot;;
var a = axel * 10000000000000;
document.write('&lt;iframe src=&quot;https://5139389.fls.doubleclick.net/activityi;src=5139389;type=fligh0;cat=fligh001;u13=homepage;u15='+region+';dc_lat=;dc_rdid=;tag_for_child_directed_treatment=;ord=' + a + '?&quot; width=&quot;1&quot; height=&quot;1&quot; frameborder=&quot;0&quot; style=&quot;display:none&quot;>&lt;/iframe>');


try {
(function(selector, addClass, rule){
document.querySelector(selector).classList.add(addClass);
})('body > div.page-wrapper > footer > div > div.trust-logos.ng-scope > img:nth-child(3)','ng-hide',document.getElementById(&quot;tag-container-opfgayqv-1526271263814&quot;).rule);
} catch (e) {}try {
(function(selector, addClass, rule){
document.querySelector(selector).classList.add(addClass);
})('div.trust-logo.icon-trustlogo-footer-iata','hidden',document.getElementById(&quot;tag-container-qreuwnwq-1526271263814&quot;).rule);
} catch (e) {}try {
(function(selector, addClass, rule){
document.querySelector(selector).classList.add(addClass);
})('body > div.main > div.content.container-fluid.landing-page > div:nth-child(3) > div.row.footer-bottom-wrapper.mbpx20 > div.col-xs-24.col-md-16.trust-logo-wrapper > div.hidden-xs > div.trust-logo.icon-trustlogo-footer-iata','hidden',document.getElementById(&quot;tag-container-rtmdrxfk-1526271263814&quot;).rule);
} catch (e) {}








/html[@class=&quot;ng-scope&quot;]/body[@class=&quot;ts-in-i-frame ltr&quot;]/div[@class=&quot;page-wrapper&quot;]/main[@class=&quot;content ng-scope&quot;]/div[@class=&quot;content-wrapper ng-scope&quot;]/section[@class=&quot;search&quot;]/section[1]/search-form[@class=&quot;ng-isolate-scope&quot;]/div[@class=&quot;ng-scope&quot;]/div[@class=&quot;search-form search-form--horizontal ng-scope&quot;]/div[@class=&quot;search-form__body&quot;]/div[@class=&quot;search-form__row&quot;]/div[@class=&quot;search-form__itins&quot;]/div[@class=&quot;ng-scope search-form__itin--last&quot;]/div[@class=&quot;search-form__itin&quot;]/div[@class=&quot;search-form__locations&quot;]/div[@class=&quot;search-form__origin&quot;]/ts-search-airports-depart[@class=&quot;ng-isolate-scope&quot;]/div[@class=&quot;airports-inline-wrapper&quot;]/div[@class=&quot;rendered-input__wrapper search-form__airport--wrapper search-form__input&quot;]/label[@class=&quot;rendered-input__label animated fadeIn ng-scope search-form__input&quot;]/div[@class=&quot;search-form__origin search-form__airport-action search-form__input&quot;]/span[@class=&quot;text-group&quot;]try {
(function(account, cat, act, label, value, nonInteraction, rule){
var val  = isNaN(parseFloat(value)) ? undefined : parseFloat(value);

if (typeof(ga) == &quot;undefined&quot;) {
  
  (function(i, s, o, g, r, a, m){
    i['GoogleAnalyticsObject'] = r; 

    i[r] = i[r] || function() {
      (i[r].q = i[r].q || []).push(arguments)
    },    i[r].l = 1 * new Date();
  
    a = s.createElement(o),
    m = s.getElementsByTagName(o)[0];
    a.async = 1;
    a.src = g;
    m.parentNode.insertBefore(a, m)  
  })(window, document, 'script', '//www.google-analytics.com/analytics.js', 'ga'); 
  
  ga('create', account, 'auto');
}


var o = {
  'hitType': 'event',
  'eventCategory': cat,
  'eventAction': act
}
if( label ){  
	o.eventLabel = label;
}
if( value ){
  o.eventValue = value;
}

if( nonInteraction ){
	o.nonInteraction = nonInteraction;
}
    
ga('send', o);

})('UA-31275-56','/','Trip Type','One way','',false,document.getElementById(&quot;tag-container-uohqdfum-1526271265660&quot;).rule);
} catch (e) {}</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;ng-scope&quot;]/body[@class=&quot;ts-in-i-frame ltr&quot;]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>ref_element</name>
      <type>Main</type>
      <value>Object Repository/Page_Flights (2)/iframe_travelstartIframe-dd7d2</value>
   </webElementProperties>
</WebElementEntity>
