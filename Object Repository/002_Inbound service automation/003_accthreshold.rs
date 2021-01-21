<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>003_accthreshold</name>
   <tag></tag>
   <elementGuidId>caa951f8-7ccc-4eb6-94f0-a0437f6033bd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:cbs=&quot;http://oss.huawei.com/business/intf/webservice/cbs&quot; xmlns:msg=&quot;http://oss.huawei.com/business/intf/webservice/cbs/msg&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;cbs:WorkOrder>
         &lt;WorkOrderRequest>
            &lt;msg:WorkOrderType>109&lt;/msg:WorkOrderType>
            &lt;msg:SubscriberNo>123456789&lt;/msg:SubscriberNo>
            &lt;msg:operationCode>uvs&lt;/msg:operationCode>
            &lt;msg:password>******&lt;/msg:password>
            &lt;msg:AccessMode>1&lt;/msg:AccessMode>
            &lt;msg:SerialNo>1308733110109&lt;/msg:SerialNo>
            &lt;!--Optional:-->
            &lt;msg:ParaList> 
               &lt;msg:ParaItem> 
                  &lt;msg:ParaName>SubCosID&lt;/msg:ParaName> 
                  &lt;msg:ParaValue>500033&lt;/msg:ParaValue> 
               &lt;/msg:ParaItem> 
               &lt;msg:ParaItem> 
                  &lt;msg:ParaName>CurrentPPSBalance&lt;/msg:ParaName> 
                  &lt;msg:ParaValue>1000000&lt;/msg:ParaValue> 
               &lt;/msg:ParaItem> 
               &lt;msg:ParaItem> 
                  &lt;msg:ParaName>CurrentPOSBalance&lt;/msg:ParaName> 
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue> 
               &lt;/msg:ParaItem> 
               &lt;msg:ParaItem> 
                  &lt;msg:ParaName>CumulateID&lt;/msg:ParaName> 
                  &lt;msg:ParaValue>45037&lt;/msg:ParaValue> 
               &lt;/msg:ParaItem> 
               &lt;msg:ParaItem> 
                  &lt;msg:ParaName>CumUsageAmount&lt;/msg:ParaName> 
                  &lt;msg:ParaValue>6297600&lt;/msg:ParaValue> 
               &lt;/msg:ParaItem> 
               &lt;msg:ParaItem> 
                  &lt;msg:ParaName>ThresholdAmount&lt;/msg:ParaName> 
                  &lt;msg:ParaValue>2097152&lt;/msg:ParaValue> 
               &lt;/msg:ParaItem> 
               &lt;msg:ParaItem> 
                  &lt;msg:ParaName>CurrentTime&lt;/msg:ParaName> 
                  &lt;msg:ParaValue>20201201000917&lt;/msg:ParaValue> 
               &lt;/msg:ParaItem> 
            &lt;/msg:ParaList> 
            &lt;msg:GroupIdList> 
               &lt;!-- msg:GroupId>1234&lt;/msg:GroupId--> 
            &lt;/msg:GroupIdList> 
         &lt;/WorkOrderRequest>
      &lt;/cbs:WorkOrder>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${accthreshold_Url}</soapServiceEndpoint>
   <soapServiceFunction>Work</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.accthreshold_Url</defaultValue>
      <description></description>
      <id>90cbcdb3-cc42-4635-98b6-5cfb3eae57ce</id>
      <masked>false</masked>
      <name>accthreshold_Url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()







GlobalVariable.variable</verificationScript>
   <wsdlAddress>file:/D:/shiva/Testing/014_Load%20balancer/AccumulativeThreshold/accthreshold.wsdl.xml</wsdlAddress>
</WebServiceRequestEntity>
