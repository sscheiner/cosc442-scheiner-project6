<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Quick Booking</name>
   <tag></tag>
   <elementGuidId>42d4d3ae-9636-4c08-a306-ddd4eeb57810</elementGuidId>
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
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
  
    
    
       Quick Booking
    
  
    
    
      
         Bookings
      
    
  
    








  td span {width: 100%;font-weight: bold;}



  Recent Bookings
     
     
        
        
            
        
        
             
                
                 
        

        
        
            
                 #↓ IDRef No.CustomerModuleDateTotalPaidRemainingStatus             
            
                1746813Selenium_20180119#5382tours19/01/201858874.11058874.11Unpaid2736897DVhbCERvhotels19/01/2018109247.30109247.3Unpaid3713018DVhbCERvtours19/01/2018357.50357.5Unpaid4701248DVhbCERvhotels19/01/2018175.10175.1Unpaid5696917DVhbCERvhotels19/01/2018593.250593.25Unpaid6688213DVhbCERvtours19/01/2018357.50357.5Unpaid7674203DVhbCERvhotels19/01/2018175.10175.1Unpaid8661708DVhbCERvtours19/01/2018357.50357.5Unpaid9659802DVhbCERvhotels19/01/2018175.10175.1Unpaid10647895DVhbCERvtours19/01/201837162.13037162.13Unpaid            
            
                            
        
        
        
            102550100All            12345            SearchAll fieldsIDRef No.CustomerModuleStatusGo            Execution time: 0.019 sMemory usage: 0.36 MB        
        
        
    

            
            &lt;!--
            
           	var xcrud_config = {&quot;url&quot;:&quot;https:\/\/www.phptravels.net\/xcrud_ajax&quot;,&quot;editor_url&quot;:false,&quot;editor_init_url&quot;:false,&quot;force_editor&quot;:false,&quot;date_first_day&quot;:1,&quot;date_format&quot;:&quot;dd.mm.yy&quot;,&quot;time_format&quot;:&quot;HH:mm:ss&quot;,&quot;lang&quot;:{&quot;add&quot;:&quot;Add&quot;,&quot;edit&quot;:&quot;Edit&quot;,&quot;view&quot;:&quot;View&quot;,&quot;remove&quot;:&quot;Remove&quot;,&quot;duplicate&quot;:&quot;Duplicate&quot;,&quot;print&quot;:&quot;Print&quot;,&quot;export_csv&quot;:&quot;Export into CSV&quot;,&quot;search&quot;:&quot;Search&quot;,&quot;go&quot;:&quot;Go&quot;,&quot;reset&quot;:&quot;Reset&quot;,&quot;save&quot;:&quot;Save&quot;,&quot;save_return&quot;:&quot;Save &amp; Return&quot;,&quot;save_new&quot;:&quot;Save &amp; New&quot;,&quot;save_edit&quot;:&quot;Save &amp; Edit&quot;,&quot;return&quot;:&quot;Return&quot;,&quot;modal_dismiss&quot;:&quot;Close&quot;,&quot;add_image&quot;:&quot;Add image&quot;,&quot;add_file&quot;:&quot;Add file&quot;,&quot;exec_time&quot;:&quot;Execution time:&quot;,&quot;memory_usage&quot;:&quot;Memory usage:&quot;,&quot;bool_on&quot;:&quot;Yes&quot;,&quot;bool_off&quot;:&quot;No&quot;,&quot;no_file&quot;:&quot;no file&quot;,&quot;no_image&quot;:&quot;no image&quot;,&quot;null_option&quot;:&quot;- none -&quot;,&quot;total_entries&quot;:&quot;Total entries:&quot;,&quot;table_empty&quot;:&quot;Entries not found.&quot;,&quot;all&quot;:&quot;All&quot;,&quot;deleting_confirm&quot;:&quot;Do you really want remove this entry?&quot;,&quot;undefined_error&quot;:&quot;It looks like something went wrong...&quot;,&quot;validation_error&quot;:&quot;Some fields are likely to contain errors. Fix errors and try again.&quot;,&quot;image_type_error&quot;:&quot;This image type is not supported.&quot;,&quot;unique_error&quot;:&quot;Some fields are not unique.&quot;,&quot;your_position&quot;:&quot;Your position&quot;,&quot;search_here&quot;:&quot;Search here...&quot;,&quot;all_fields&quot;:&quot;All fields&quot;,&quot;choose_range&quot;:&quot;- choose range -&quot;,&quot;next_year&quot;:&quot;Next year&quot;,&quot;next_month&quot;:&quot;Next month&quot;,&quot;today&quot;:&quot;Today&quot;,&quot;this_week_today&quot;:&quot;This week up to today&quot;,&quot;this_week_full&quot;:&quot;This full week&quot;,&quot;last_week&quot;:&quot;Last week&quot;,&quot;last_2weeks&quot;:&quot;Last two weeks&quot;,&quot;this_month&quot;:&quot;This month&quot;,&quot;last_month&quot;:&quot;Last month&quot;,&quot;last_3months&quot;:&quot;Last 3 months&quot;,&quot;last_6months&quot;:&quot;Last 6 months&quot;,&quot;this_year&quot;:&quot;This year&quot;,&quot;last_year&quot;:&quot;Last year&quot;},&quot;rtl&quot;:0};
                            
            -->
               
 







  
      



  
    
      
        
          ×
          Quick Booking
        
        
          
            Apply Tax
            
              
                Yes
                No
              
            
          
          
            Service
            
              
                Select
                                Tours
                                Hotels
                              
            
          
        
        
          Close
          Next
        
      
    
  






  $(function () {

    $(&quot;.resetChart&quot;).on(&quot;click&quot;,function(){

  $.alert.open('confirm', 'Are you sure you want to reset visits data?', function(answer) {
   if(answer == 'yes'){

      $.post(&quot;https://www.phptravels.net/admin/resetVisits&quot;,{isAjax: &quot;yes&quot;},function(resp){

       location.reload();

      })

    }else{

      return false;
    }

    });

});


          $('#resgraph').highcharts({
                  chart: {
              type: 'column',
               zoomType: 'x'
          },
          title: {
              text: ''
          },
          xAxis: {
            title: {
                  text: &quot;May&quot;
              },
            categories: []
          },
          yAxis: {
              min: 0,
              title: {
                  text: 'Visits'
              },
              stackLabels: {
                  enabled: true,
                  style: {
                      fontWeight: 'bold',
                      color: (Highcharts.theme &amp;&amp; Highcharts.theme.textColor) || 'gray'
                  }
              }
          },
          legend: {
              align: 'right',
              x: -70,
              verticalAlign: 'top',
              y: 20,
              floating: true,
              backgroundColor: (Highcharts.theme &amp;&amp; Highcharts.theme.background2) || 'white',
              borderColor: '#CCC',
              borderWidth: 1,
              shadow: false
          },
          tooltip: {
              formatter: function () {
                  return '&lt;b>' + this.x + '&lt;/b>&lt;br/>' +
                      this.series.name + ': ' + this.y + '&lt;br/>' +
                      'Total: ' + this.point.stackTotal;
              }
          },
          plotOptions: {
              column: {
                  stacking: 'normal',
                  dataLabels: {
                      enabled: false,
                      color: (Highcharts.theme &amp;&amp; Highcharts.theme.dataLabelsColor) || 'white',
                      style: {
                          textShadow: '0 0 3px black, 0 0 3px black'
                      }
                  }
              }
          },
          series: [{
              name: 'Unique',
              data: [],
       stack: 'male'
          }, {
              name: 'Total Hits',
              data: [],
      stack: 'female'
          }]
          });
      });



</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;pace-done&quot;]/div[@class=&quot;wrapper&quot;]/div[1]</value>
   </webElementProperties>
</WebElementEntity>
