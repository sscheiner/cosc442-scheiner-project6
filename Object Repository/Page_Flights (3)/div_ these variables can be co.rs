<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_ these variables can be co</name>
   <tag></tag>
   <elementGuidId>e334fc3f-1c89-4be8-9d3b-fc756cb0598f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>container</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>






	// these variables can be configured
	var travelstartIframeId = 'travelstartIframe-dd7d2f33-38c3-4c69-baac-56d16157023b';
	var iframeUrl = 'https://www.travelstart.ae';
	var logMessages = false;
	var showBanners = true;
	var affId = '189737';
	var affCampaign = '';
	var affCurrency = 'Default'; // ZAR / USD / NAD / ...
	var height = '0px';
	var width = '100%';
	var language = 'en'; // ar / en / leave empty for user preference

	// do not change these
	var iframe = $('#' + travelstartIframeId);
	var iframeVersion = '10';
	var autoSearch = false;
	var affiliateIdExist = true;
	var urlParams = {};
	var alreadyExist = [];
	var iframeParams = [];
	var cpySource = '';
	var match,
		pl = /\+/g,
		search = /([^&amp;=]+)=?([^&amp;]*)/g,
		decode = function (s) { return decodeURIComponent(s.replace(pl, &quot; &quot;)); },
		query  = window.location.search.substring(1);
	while (match = search.exec(query)){
		urlParams[decode(match[1])] = decode(match[2]);
	}
	for (var key in urlParams){
		if (urlParams.hasOwnProperty(key)){
			if (key == 'search' &amp;&amp; urlParams[key] == 'true'){
				autoSearch = true;
			}
			if(	key == 'affId' || key == 'affid' || key == 'aff_id'){
				affiliateIdExist = true ;
			}
			iframeParams.push(key + '=' + urlParams[key]);
			alreadyExist.push(key);
		}
	}
  	if(!('show_banners' in alreadyExist)){
		iframeParams.push('show_banners=' + showBanners);
	}
	if(!('log' in alreadyExist)){
		iframeParams.push('log='  + logMessages);
	}
	if(! affiliateIdExist){
		iframeParams.push('affId='  + affId);
	}
	if(! affiliateIdExist){
		iframeParams.push('language='  + language);
	}
	if(!('affCampaign' in alreadyExist)){
		iframeParams.push('affCampaign='  + affCampaign);
	}
	if(cpySource !== '' &amp;&amp; !('cpySource' in alreadyExist)){
		iframeParams.push('cpy_source='  + cpySource);
	}
	if(!('utm_source' in alreadyExist)){
		iframeParams.push('utm_source=affiliate');
	}
	if(!('utm_medium' in alreadyExist)){
		iframeParams.push('utm_medium='  + affId);
	}
	if(!('isiframe' in alreadyExist)){
		iframeParams.push('isiframe=true');
	}
	if(!('landing_page' in alreadyExist)){
		iframeParams.push('landing_page=false');
	}
	if (affCurrency.length == 3){
		iframeParams.push('currency=' + affCurrency);
	}
	if(!('iframeVersion' in alreadyExist)){
   	iframeParams.push('iframeVersion='  + iframeVersion);
	}
	if(!('host' in alreadyExist)){
		iframeParams.push('host=' + window.location.href.split('?')[0]);
	}
	var newIframeUrl = iframeUrl + (autoSearch ? '/search-on-index?search=true' : '/search-on-index?search=false') + '&amp;' + iframeParams.join('&amp;');
	iframe.attr('src', newIframeUrl);
	function setIframeSize(newWidth, newHeight){
		iframe.css('width', newWidth);
		iframe.width(newWidth);
		iframe.css('height', newHeight);
		iframe.height(newHeight);
	}
	setIframeSize(width, height);
	$.receiveMessage(function(e, host){
		if (logMessages){
			$('#logs').text('RECEIVED *** ' + new Date() + ' *** ' + 'message=' + e.data + ' *** iframeUrl=' + newIframeUrl);
		}
  	    var dataElements = e.data.split('&amp;');
  	    if(dataElements &amp;&amp; dataElements.length === 1) {
  	       setIframeSize(width, e.data + 'px');
  	    } else {
  	       var elementKey = dataElements[0].split('=');
  	       var elementValue = dataElements[1].split('=');
  	       if(elementKey[1] === 'resize') {
  	          setIframeSize(width, elementValue[1] + 'px');
  	       }
  	       if(elementKey[1] === 'deeplink') {
  	          window.location.replace(unescape(elementValue[1]));
  	       }
  	    }
   }, iframeUrl);

</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;body-section&quot;)/div[@class=&quot;container&quot;]</value>
   </webElementProperties>
</WebElementEntity>
